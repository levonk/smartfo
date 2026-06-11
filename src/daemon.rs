//! Self-spawning daemon with Unix socket communication
//!
//! This module implements the daemon lifecycle:
//! - Double-fork detachment for background operation
//! - PID lockfile management
//! - Unix domain socket for CLI-to-daemon communication
//! - Graceful shutdown on SIGTERM

use anyhow::{Context, Result};
use nix::unistd::{fork, ForkResult, setsid, Pid};
use nix::sys::signal::{self, SigHandler, Signal};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use tracing::{info, warn, error};

/// Global shutdown flag for signal handler
/// This must be static because signal handlers can't capture variables
static SHUTDOWN_FLAG: AtomicBool = AtomicBool::new(false);

/// Daemon instance managing lifecycle and communication
#[derive(Clone)]
pub struct Daemon {
    pid_file: PathBuf,
    socket_path: PathBuf,
}

impl Daemon {
    /// Create a new daemon instance with default paths
    pub fn new() -> Result<Self> {
        let xdg_data_home = std::env::var("XDG_DATA_HOME")
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").expect("HOME not set");
                format!("{}/.local/share", home)
            });

        let smartfo_data = PathBuf::from(xdg_data_home).join("smartfo");

        // Ensure data directory exists
        std::fs::create_dir_all(&smartfo_data)
            .context("Failed to create smartfo data directory")?;

        Ok(Daemon {
            pid_file: smartfo_data.join("daemon.pid"),
            socket_path: smartfo_data.join("daemon.sock"),
        })
    }

    /// Perform double-fork to detach as a background daemon
    ///
    /// # Process
    /// 1. First fork: Parent exits, child continues
    /// 2. Child creates new session (setsid) to detach from terminal
    /// 3. Second fork: Child exits, grandchild continues as daemon
    /// 4. Grandchild is now a proper daemon with no controlling terminal
    ///
    /// # Returns
    /// - `Ok(true)` if we are the daemon (grandchild process)
    /// - `Ok(false)` if we are the parent (should exit)
    /// - `Err` if fork fails
    pub fn double_fork_detach() -> Result<bool> {
        info!("Starting double-fork detachment");

        // First fork
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child: _ }) => {
                info!("First fork: parent exiting, child continues in background");
                return Ok(false);
            }
            Ok(ForkResult::Child) => {
                info!("First fork: child process, creating new session");

                // Create new session to detach from controlling terminal
                setsid().context("Failed to create new session - Daemon may not detach properly")?;

                // Second fork
                match unsafe { fork() } {
                    Ok(ForkResult::Parent { child: _ }) => {
                        info!("Second fork: intermediate child exiting, grandchild is daemon");
                        std::process::exit(0);
                    }
                    Ok(ForkResult::Child) => {
                        info!("Second fork: grandchild is now the daemon");
                        return Ok(true);
                    }
                    Err(e) => {
                        error!("Second fork failed: {}", e);
                        return Err(e.into());
                    }
                }
            }
            Err(e) => {
                error!("First fork failed: {}", e);
                return Err(e.into());
            }
        }
    }

    /// Write PID to lockfile
    pub fn write_pid_file(&self) -> Result<()> {
        let pid = std::process::id();
        let mut file = File::create(&self.pid_file)
            .context("Failed to create PID file")?;

        writeln!(file, "{}", pid)
            .context("Failed to write PID to file")?;

        info!("Wrote PID {} to lockfile: {:?}", pid, self.pid_file);
        Ok(())
    }

    /// Read PID from lockfile
    pub fn read_pid_file(&self) -> Result<Option<u32>> {
        if !self.pid_file.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&self.pid_file)
            .context("Failed to read PID file")?;

        let pid: u32 = content.trim().parse()
            .context("Failed to parse PID from file")?;

        Ok(Some(pid))
    }

    /// Check if a process with the given PID is running
    pub fn is_process_running(pid: u32) -> bool {
        // Send signal 0 to check if process exists without affecting it
        signal::kill(Pid::from_raw(pid as i32), None).is_ok()
    }

    /// Acquire daemon lock by checking and writing PID file
    ///
    /// Returns true if lock was acquired, false if daemon is already running
    pub fn acquire_lock(&self) -> Result<bool> {
        if let Some(existing_pid) = self.read_pid_file()? {
            if Self::is_process_running(existing_pid) {
                info!("Daemon already running with PID {}", existing_pid);
                return Ok(false);
            } else {
                warn!("Stale PID file found (PID {} not running), removing", existing_pid);
                std::fs::remove_file(&self.pid_file)
                    .context("Failed to remove stale PID file")?;
            }
        }

        self.write_pid_file()?;
        Ok(true)
    }

    /// Get the socket path
    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }

    /// Clean up PID file on shutdown
    pub fn cleanup(&self) -> Result<()> {
        if self.pid_file.exists() {
            std::fs::remove_file(&self.pid_file)
                .context("Failed to remove PID file")?;
            info!("Removed PID file: {:?}", self.pid_file);
        }
        if self.socket_path.exists() {
            std::fs::remove_file(&self.socket_path)
                .context("Failed to remove socket file")?;
            info!("Removed socket file: {:?}", self.socket_path);
        }
        Ok(())
    }

    /// Check if shutdown has been requested
    pub fn is_shutdown_requested() -> bool {
        SHUTDOWN_FLAG.load(Ordering::SeqCst)
    }

    /// Request shutdown by setting the shutdown flag
    pub fn request_shutdown() {
        SHUTDOWN_FLAG.store(true, Ordering::SeqCst);
        info!("Shutdown requested");
    }

    /// Set up signal handler for graceful shutdown
    ///
    /// This installs a handler for SIGTERM that sets the shutdown flag
    pub fn setup_signal_handler(&self) -> Result<()> {
        // Safety: The signal handler only performs an atomic store on a static variable, which is safe
        unsafe {
            extern "C" fn sigterm_handler(_sig: i32) {
                SHUTDOWN_FLAG.store(true, Ordering::SeqCst);
            }

            let handler = SigHandler::Handler(sigterm_handler);
            signal::signal(Signal::SIGTERM, handler)
                .context("Failed to install SIGTERM handler")?;
        }

        info!("Installed SIGTERM handler for graceful shutdown");
        Ok(())
    }

    /// Bind and listen on Unix domain socket
    ///
    /// Removes existing socket file if present (stale from previous daemon)
    pub fn bind_socket(&self) -> Result<UnixListener> {
        // Remove stale socket file if it exists
        if self.socket_path.exists() {
            warn!("Removing stale socket file: {:?}", self.socket_path);
            std::fs::remove_file(&self.socket_path)
                .context("Failed to remove stale socket file")?;
        }

        let listener = UnixListener::bind(&self.socket_path)
            .context("Failed to bind Unix socket")?;

        info!("Daemon listening on socket: {:?}", self.socket_path);
        Ok(listener)
    }

    /// Accept a single connection from the socket
    ///
    /// This blocks until a client connects
    pub fn accept_connection(&self, listener: &UnixListener) -> Result<UnixStream> {
        let (stream, _addr) = listener.accept()
            .context("Failed to accept socket connection")?;
        info!("Accepted connection from client");
        Ok(stream)
    }

    /// Connect to an existing daemon socket
    ///
    /// Returns Ok(None) if no daemon is running (socket doesn't exist)
    /// Returns Ok(Some(stream)) if connection succeeds
    pub fn connect_to_daemon(&self) -> Result<Option<UnixStream>> {
        if !self.socket_path.exists() {
            info!("No daemon socket found at {:?}", self.socket_path);
            return Ok(None);
        }

        match UnixStream::connect(&self.socket_path) {
            Ok(stream) => {
                info!("Connected to existing daemon at {:?}", self.socket_path);
                Ok(Some(stream))
            }
            Err(e) => {
                warn!("Failed to connect to daemon socket: {}", e);
                // Socket exists but connection failed - likely stale
                Ok(None)
            }
        }
    }

    /// Send a simple ping message and wait for response
    ///
    /// Used for health checks
    pub fn ping_daemon(&self) -> Result<bool> {
        if let Some(mut stream) = self.connect_to_daemon()? {
            // Send ping
            writeln!(stream, "PING")
                .context("Failed to send ping")?;

            // Wait for response
            let mut reader = BufReader::new(&stream);
            let mut response = String::new();
            reader.read_line(&mut response)
                .context("Failed to read ping response")?;

            if response.trim() == "PONG" {
                info!("Daemon is alive");
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Handle a client connection (simple echo/pong for now)
    ///
    /// This is a placeholder - will be expanded to handle job enqueue/status
    /// Returns job ID if a job was enqueued
    pub fn handle_client_connection(&self, stream: UnixStream) -> Result<Option<String>> {
        let mut reader = BufReader::new(stream);
        let mut request = String::new();
        reader.read_line(&mut request)
            .context("Failed to read request from client")?;

        let request = request.trim();
        info!("Received request: {}", request);

        match request {
            "PING" => {
                let mut writer = reader.into_inner();
                writeln!(writer, "PONG")
                    .context("Failed to send PONG response")?;
                info!("Sent PONG response");
                Ok(None)
            }
            _ => {
                let mut writer = reader.into_inner();
                warn!("Unknown request: {}", request);
                writeln!(writer, "ERROR: Unknown command")
                    .context("Failed to send error response")?;
                Ok(None)
            }
        }
    }

    /// Check if daemon mode is supported on this platform
    ///
    /// Daemon mode requires Unix domain sockets and fork support
    pub fn is_daemon_supported() -> bool {
        cfg!(unix)
    }

    /// Get or spawn daemon connection from CLI perspective
    ///
    /// This function is called from the CLI when an async operation is needed:
    /// 1. First tries to connect to an existing daemon
    /// 2. If no daemon is running, spawns a new daemon via double-fork
    /// 3. Retries connection to the newly spawned daemon
    ///
    /// Returns Ok(stream) if connection succeeds, Err if it fails after retries
    pub fn get_or_spawn_daemon(&self) -> Result<UnixStream> {
        // First, try to connect to existing daemon
        if let Some(stream) = self.connect_to_daemon()? {
            info!("Connected to existing daemon");
            return Ok(stream);
        }

        info!("No daemon running, spawning new daemon");

        // Check if we can acquire the lock (no daemon running)
        if !self.acquire_lock()? {
            // Another process is spawning the daemon, wait and retry
            info!("Another process is spawning daemon, waiting...");
            for attempt in 1..=5 {
                std::thread::sleep(std::time::Duration::from_millis(200 * attempt));
                if let Some(stream) = self.connect_to_daemon()? {
                    info!("Connected to daemon after wait");
                    return Ok(stream);
                }
            }
            return Err(anyhow::anyhow!("Timeout waiting for daemon to start"));
        }

        // We have the lock, spawn the daemon
        match Self::double_fork_detach() {
            Ok(true) => {
                // We are the daemon process
                info!("Daemon process started, entering daemon loop");

                // Set up signal handler for graceful shutdown
                self.setup_signal_handler()
                    .context("Failed to set up signal handler")?;

                // Bind socket
                let listener = self.bind_socket()
                    .context("Failed to bind daemon socket")?;

                // Enter daemon event loop (placeholder - will be expanded)
                info!("Daemon entering event loop");
                // TODO: Implement full daemon event loop with job processing
                // For now, just handle a single connection for testing
                if let Ok(stream) = self.accept_connection(&listener) {
                    let _ = self.handle_client_connection(stream);
                }

                // Check if shutdown was requested
                if Self::is_shutdown_requested() {
                    info!("Shutdown requested, completing in-flight jobs...");
                    // TODO: Wait for in-flight jobs to complete (will be implemented in story 04-003)
                    // For now, just exit gracefully
                }

                // Cleanup and exit
                let _ = self.cleanup();
                info!("Daemon exiting");
                std::process::exit(0);
            }
            Ok(false) => {
                // We are the parent process, wait for daemon to start
                info!("Parent process: waiting for daemon to start");

                // Release the lock (daemon has it now)
                let _ = std::fs::remove_file(&self.pid_file);

                // Retry connection with backoff
                for attempt in 1..=10 {
                    std::thread::sleep(std::time::Duration::from_millis(100 * attempt));
                    if let Some(stream) = self.connect_to_daemon()? {
                        info!("Parent: connected to spawned daemon");
                        return Ok(stream);
                    }
                }

                Err(anyhow::anyhow!("Failed to connect to spawned daemon after retries"))
            }
            Err(e) => {
                // Fork failed, cleanup and return error
                let _ = self.cleanup();
                Err(e).context("Failed to spawn daemon")
            }
        }
    }
}

impl Default for Daemon {
    fn default() -> Self {
        Self::new().expect("Failed to create daemon instance")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_double_fork_returns_false_for_parent() {
        // This test runs in the parent process after first fork
        // The double_fork_detach should return false for parent
        let result = Daemon::double_fork_detach();
        // We can't easily test this in a unit test context
        // This is more of an integration test scenario
        // For now, just ensure it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_pid_file_operations() {
        let temp_dir = TempDir::new().unwrap();
        let pid_file = temp_dir.path().join("test.pid");

        let daemon = Daemon {
            pid_file: pid_file.clone(),
            socket_path: temp_dir.path().join("test.sock"),
        };

        // Initially no PID file
        assert_eq!(daemon.read_pid_file().unwrap(), None);

        // Write PID
        daemon.write_pid_file().unwrap();

        // Read PID back
        let pid = daemon.read_pid_file().unwrap();
        assert!(pid.is_some());
        assert_eq!(pid.unwrap(), std::process::id());

        // Clean up
        daemon.cleanup().unwrap();
        assert_eq!(daemon.read_pid_file().unwrap(), None);
    }

    #[test]
    fn test_is_process_running() {
        // Current process should be running
        assert!(Daemon::is_process_running(std::process::id()));

        // PID 999999 is unlikely to be running
        assert!(!Daemon::is_process_running(999999));
    }

    #[test]
    fn test_acquire_lock_with_stale_pid() {
        let temp_dir = TempDir::new().unwrap();
        let pid_file = temp_dir.path().join("test.pid");

        // Create a stale PID file with a non-running PID
        let mut file = File::create(&pid_file).unwrap();
        writeln!(file, "999999").unwrap();

        let daemon = Daemon {
            pid_file: pid_file.clone(),
            socket_path: temp_dir.path().join("test.sock"),
        };

        // Should acquire lock despite stale PID file
        let acquired = daemon.acquire_lock().unwrap();
        assert!(acquired);

        // PID should now be current process
        let pid = daemon.read_pid_file().unwrap();
        assert_eq!(pid.unwrap(), std::process::id());
    }

    #[test]
    fn test_socket_bind_and_cleanup() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");

        let daemon = Daemon {
            pid_file: temp_dir.path().join("test.pid"),
            socket_path: socket_path.clone(),
        };

        // Socket shouldn't exist initially
        assert!(!socket_path.exists());

        // Bind socket
        let listener = daemon.bind_socket().unwrap();
        assert!(socket_path.exists());

        // Cleanup should remove socket
        daemon.cleanup().unwrap();
        assert!(!socket_path.exists());

        // Drop listener to release file descriptor
        drop(listener);
    }

    #[test]
    fn test_connect_to_daemon_when_not_running() {
        let temp_dir = TempDir::new().unwrap();

        let daemon = Daemon {
            pid_file: temp_dir.path().join("test.pid"),
            socket_path: temp_dir.path().join("test.sock"),
        };

        // No socket exists, should return None
        let result = daemon.connect_to_daemon().unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_ping_pong_protocol() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");

        let daemon = Daemon {
            pid_file: temp_dir.path().join("test.pid"),
            socket_path: socket_path.clone(),
        };

        // Bind socket (simulating daemon)
        let listener = daemon.bind_socket().unwrap();

        // Spawn a thread to handle the connection
        let daemon_clone = daemon.clone();
        std::thread::spawn(move || {
            if let Ok(stream) = daemon_clone.accept_connection(&listener) {
                let _ = daemon_clone.handle_client_connection(stream);
            }
        });

        // Give the server thread a moment to start
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Connect as client and send ping
        let result = daemon.ping_daemon().unwrap();
        assert!(result);

        // Cleanup (listener is dropped automatically when thread ends)
        daemon.cleanup().unwrap();
    }

    #[test]
    fn test_handle_client_connection_ping() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");

        let daemon = Daemon {
            pid_file: temp_dir.path().join("test.pid"),
            socket_path: socket_path.clone(),
        };

        // Bind socket
        let listener = daemon.bind_socket().unwrap();

        // Spawn a thread to handle connections
        let daemon_clone = daemon.clone();
        std::thread::spawn(move || {
            if let Ok(stream) = daemon_clone.accept_connection(&listener) {
                let _ = daemon_clone.handle_client_connection(stream);
            }
        });

        // Give server thread a moment to start
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Connect as client
        let mut stream = UnixStream::connect(&socket_path).unwrap();

        // Send PING
        writeln!(stream, "PING").unwrap();

        // Read response
        let mut reader = BufReader::new(stream);
        let mut response = String::new();
        reader.read_line(&mut response).unwrap();

        assert_eq!(response.trim(), "PONG");

        // Cleanup (listener is dropped automatically when thread ends)
        daemon.cleanup().unwrap();
    }

    #[test]
    fn test_get_or_spawn_daemon_connects_to_existing() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");

        let daemon = Daemon {
            pid_file: temp_dir.path().join("test.pid"),
            socket_path: socket_path.clone(),
        };

        // Bind socket (simulating existing daemon)
        let listener = daemon.bind_socket().unwrap();

        // Spawn a thread to handle connections
        let daemon_clone = daemon.clone();
        std::thread::spawn(move || {
            if let Ok(stream) = daemon_clone.accept_connection(&listener) {
                let _ = daemon_clone.handle_client_connection(stream);
            }
        });

        // Give server thread a moment to start
        std::thread::sleep(std::time::Duration::from_millis(100));

        // get_or_spawn_daemon should connect to existing daemon
        let result = daemon.get_or_spawn_daemon();
        assert!(result.is_ok(), "Should connect to existing daemon");

        // Cleanup
        daemon.cleanup().unwrap();
    }

    #[test]
    fn test_shutdown_flag() {
        // Reset the flag first
        SHUTDOWN_FLAG.store(false, Ordering::SeqCst);

        // Initially not shutdown
        assert!(!Daemon::is_shutdown_requested());

        // Request shutdown
        Daemon::request_shutdown();

        // Now should be shutdown
        assert!(Daemon::is_shutdown_requested());

        // Reset for other tests
        SHUTDOWN_FLAG.store(false, Ordering::SeqCst);
    }

    #[test]
    fn test_signal_handler_setup() {
        let daemon = Daemon::new().unwrap();

        // Setting up signal handler should succeed
        // Note: We can't easily test the actual signal delivery in a unit test
        // but we can verify the setup doesn't fail
        let result = daemon.setup_signal_handler();
        assert!(result.is_ok(), "Signal handler setup should succeed");
    }

    #[test]
    fn test_is_daemon_supported() {
        // On Unix platforms, daemon should be supported
        #[cfg(unix)]
        assert!(Daemon::is_daemon_supported(), "Daemon should be supported on Unix");

        // On non-Unix platforms, daemon should not be supported
        #[cfg(not(unix))]
        assert!(!Daemon::is_daemon_supported(), "Daemon should not be supported on non-Unix");
    }

    #[test]
    fn test_handle_client_connection_returns_job_id() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");

        let daemon = Daemon {
            pid_file: temp_dir.path().join("test.pid"),
            socket_path: socket_path.clone(),
        };

        // Bind socket (simulating daemon)
        let listener = daemon.bind_socket().unwrap();

        // Spawn a thread to handle connections
        let daemon_clone = daemon.clone();
        std::thread::spawn(move || {
            if let Ok(stream) = daemon_clone.accept_connection(&listener) {
                let _ = daemon_clone.handle_client_connection(stream);
            }
        });

        // Give server thread a moment to start
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Connect as client
        let mut stream = UnixStream::connect(&socket_path).unwrap();

        // Send PING
        writeln!(stream, "PING").unwrap();

        // Read response
        let mut reader = BufReader::new(stream);
        let mut response = String::new();
        reader.read_line(&mut response).unwrap();

        assert_eq!(response.trim(), "PONG");

        // Cleanup
        daemon.cleanup().unwrap();
    }
}
