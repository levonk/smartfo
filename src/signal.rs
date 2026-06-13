use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::{error, info, warn};

/// Global flag for config reload request
static RELOAD_REQUESTED: AtomicBool = AtomicBool::new(false);

/// Initialize signal handlers for the current process
pub fn init_signal_handlers() -> Result<()> {
    #[cfg(unix)]
    {
        use signal_hook::{consts::SIGHUP, iterator::Signals};
        
        let mut signals = Signals::new([SIGHUP])?;
        
        // Spawn a thread to handle signals
        std::thread::spawn(move || {
            for sig in signals.forever() {
                match sig {
                    SIGHUP => {
                        info!("Received SIGHUP signal, requesting config reload");
                        RELOAD_REQUESTED.store(true, Ordering::SeqCst);
                    }
                    _ => {
                        warn!("Received unexpected signal: {}", sig);
                    }
                }
            }
        });
        
        info!("Signal handlers initialized (SIGHUP for config reload)");
    }
    
    #[cfg(not(unix))]
    {
        info!("Signal handlers not available on this platform");
    }
    
    Ok(())
}

/// Check if a config reload has been requested
pub fn is_reload_requested() -> bool {
    RELOAD_REQUESTED.load(Ordering::SeqCst)
}

/// Clear the reload request flag
pub fn clear_reload_request() {
    RELOAD_REQUESTED.store(false, Ordering::SeqCst);
}

/// Request a config reload programmatically
pub fn request_reload() {
    info!("Programmatic config reload requested");
    RELOAD_REQUESTED.store(true, Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reload_request_flag() {
        // Clear any existing request
        clear_reload_request();
        
        // Initially should be false
        assert!(!is_reload_requested());
        
        // Request reload
        request_reload();
        
        // Should now be true
        assert!(is_reload_requested());
        
        // Clear the request
        clear_reload_request();
        
        // Should be false again
        assert!(!is_reload_requested());
    }

    #[test]
    fn test_clear_reload_request() {
        // Set the flag
        request_reload();
        assert!(is_reload_requested());
        
        // Clear it
        clear_reload_request();
        assert!(!is_reload_requested());
        
        // Clearing again should be safe
        clear_reload_request();
        assert!(!is_reload_requested());
    }
}
