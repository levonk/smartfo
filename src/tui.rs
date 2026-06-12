//! TUI (Terminal User Interface) module for smartfo
//!
//! Provides interactive terminal-based UI for viewing and modifying arguments,
//! configuration, and complex operations.

use anyhow::{Result, Context};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::time::Duration;

/// TUI mode configuration
#[derive(Debug, Clone)]
pub struct TuiConfig {
    /// Enable mouse support
    pub mouse_enabled: bool,
    /// Refresh rate in milliseconds
    pub refresh_rate_ms: u64,
    /// Maximum terminal width (for layout calculations)
    pub max_width: u16,
    /// Maximum terminal height (for layout calculations)
    pub max_height: u16,
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            mouse_enabled: false,
            refresh_rate_ms: 100,
            max_width: 120,
            max_height: 40,
        }
    }
}

/// TUI application state
#[derive(Debug, Clone)]
pub enum TuiMode {
    /// Argument editor mode
    ArgumentEditor,
    /// Config editor mode
    ConfigEditor,
    /// Install mode
    Install,
    /// Batch operations mode
    BatchOperations,
}

/// TUI application
pub struct TuiApp {
    /// Current mode
    mode: TuiMode,
    /// Configuration
    config: TuiConfig,
    /// Current input text
    input: String,
    /// Selected item index
    selected: usize,
    /// List items
    items: Vec<String>,
    /// Should quit flag
    should_quit: bool,
    /// Terminal size
    terminal_size: (u16, u16),
}

impl TuiApp {
    /// Create a new TUI application
    pub fn new(mode: TuiMode) -> Self {
        Self {
            mode,
            config: TuiConfig::default(),
            input: String::new(),
            selected: 0,
            items: Vec::new(),
            should_quit: false,
            terminal_size: (80, 24),
        }
    }

    /// Set the configuration
    pub fn with_config(mut self, config: TuiConfig) -> Self {
        self.config = config;
        self
    }

    /// Set the list items
    pub fn with_items(mut self, items: Vec<String>) -> Self {
        self.items = items;
        self
    }

    /// Set the initial input
    pub fn with_input(mut self, input: String) -> Self {
        self.input = input;
        self
    }

    /// Run the TUI application
    pub fn run(&mut self) -> Result<String> {
        // Setup terminal
        enable_raw_mode().context("Failed to enable raw mode")?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
            .context("Failed to enter alternate screen")?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;

        // Get initial terminal size
        self.terminal_size = terminal.size()
            .map(|s| (s.width, s.height))
            .unwrap_or((80, 24));

        // Main event loop
        let result = self.run_event_loop(&mut terminal);

        // Restore terminal
        disable_raw_mode().context("Failed to disable raw mode")?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .context("Failed to leave alternate screen")?;

        result
    }

    /// Run the main event loop
    fn run_event_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<String> {
        let tick_rate = Duration::from_millis(self.config.refresh_rate_ms);

        loop {
            // Draw the UI
            terminal.draw(|f| self.draw(f))?;

            // Handle events
            if event::poll(tick_rate)? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key_event(key);
                }
            }

            // Check for quit
            if self.should_quit {
                return Ok(self.input.clone());
            }
        }
    }

    /// Handle key events
    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter => {
                self.should_quit = true;
            }
            KeyCode::Esc => {
                self.should_quit = true;
                self.input.clear(); // Clear input on escape
            }
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected < self.items.len().saturating_sub(1) {
                    self.selected += 1;
                }
            }
            KeyCode::Left => {
                // Navigate left in input
            }
            KeyCode::Right => {
                // Navigate right in input
            }
            _ => {}
        }
    }

    /// Draw the UI
    fn draw(&self, f: &mut Frame) {
        let size = f.size();

        // Create layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3), // Header
                    Constraint::Min(0),    // Content
                    Constraint::Length(3), // Input
                ]
                .as_ref(),
            )
            .split(size);

        // Draw header
        self.draw_header(f, chunks[0]);

        // Draw content based on mode
        match self.mode {
            TuiMode::ArgumentEditor => self.draw_argument_editor(f, chunks[1]),
            TuiMode::ConfigEditor => self.draw_config_editor(f, chunks[1]),
            TuiMode::Install => self.draw_install(f, chunks[1]),
            TuiMode::BatchOperations => self.draw_batch_operations(f, chunks[1]),
        }

        // Draw input
        self.draw_input(f, chunks[2]);
    }

    /// Draw the header
    fn draw_header(&self, f: &mut Frame, area: Rect) {
        let title = match self.mode {
            TuiMode::ArgumentEditor => "Argument Editor",
            TuiMode::ConfigEditor => "Config Editor",
            TuiMode::Install => "Install",
            TuiMode::BatchOperations => "Batch Operations",
        };

        let header = Paragraph::new(vec![
            Line::from(Span::styled(
                title,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from("Press Enter to confirm, Esc to cancel"),
        ])
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

        f.render_widget(header, area);
    }

    /// Draw the argument editor
    fn draw_argument_editor(&self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == self.selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(Span::styled(item, style))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().title("Arguments").borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        f.render_widget(list, area);
    }

    /// Draw the config editor
    fn draw_config_editor(&self, f: &mut Frame, area: Rect) {
        let text = Text::from(vec![
            Line::from("Config Editor"),
            Line::from(""),
            Line::from("Use arrow keys to navigate"),
            Line::from("Press Enter to edit a value"),
        ]);

        let paragraph = Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .block(Block::default().title("Configuration").borders(Borders::ALL));

        f.render_widget(paragraph, area);
    }

    /// Draw the install screen
    fn draw_install(&self, f: &mut Frame, area: Rect) {
        let text = Text::from(vec![
            Line::from("Installation"),
            Line::from(""),
            Line::from("This will install smartfo symlinks and hooks"),
            Line::from(""),
            Line::from(format!("Target: {}", self.input)),
        ]);

        let paragraph = Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .block(Block::default().title("Install").borders(Borders::ALL));

        f.render_widget(paragraph, area);
    }

    /// Draw the batch operations screen
    fn draw_batch_operations(&self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == self.selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(Span::styled(item, style))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().title("Batch Operations").borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        f.render_widget(list, area);
    }

    /// Draw the input field
    fn draw_input(&self, f: &mut Frame, area: Rect) {
        let input_text = if self.input.is_empty() {
            "Type here..."
        } else {
            &self.input
        };

        let paragraph = Paragraph::new(input_text)
            .block(Block::default().title("Input").borders(Borders::ALL));

        f.render_widget(paragraph, area);
    }

    /// Handle terminal resize
    pub fn handle_resize(&mut self, width: u16, height: u16) {
        self.terminal_size = (width, height);
    }

    /// Get the current input text
    pub fn input(&self) -> &str {
        &self.input
    }

    /// Get the selected item index
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// Get the should_quit flag
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Get the config
    pub fn config(&self) -> &TuiConfig {
        &self.config
    }

    /// Get the items
    pub fn items(&self) -> &[String] {
        &self.items
    }

    /// Get the terminal size
    pub fn terminal_size(&self) -> (u16, u16) {
        self.terminal_size
    }
}

/// Launch TUI for argument editing
pub fn edit_arguments(args: Vec<String>) -> Result<String> {
    let mut app = TuiApp::new(TuiMode::ArgumentEditor)
        .with_items(args)
        .with_input(String::new());

    app.run()
}

/// Launch TUI for config editing
pub fn edit_config() -> Result<String> {
    let mut app = TuiApp::new(TuiMode::ConfigEditor);
    app.run()
}

/// Launch TUI for install operations
pub fn install_tui() -> Result<String> {
    let mut app = TuiApp::new(TuiMode::Install);
    app.run()
}

/// Launch TUI for batch operations
pub fn batch_operations(items: Vec<String>) -> Result<String> {
    let mut app = TuiApp::new(TuiMode::BatchOperations)
        .with_items(items);
    app.run()
}

/// Check if TUI is supported in the current environment
pub fn is_tui_supported() -> bool {
    // Check if we're in a terminal
    atty::is(atty::Stream::Stdout) && atty::is(atty::Stream::Stdin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tui_config_default() {
        let config = TuiConfig::default();
        assert!(!config.mouse_enabled);
        assert_eq!(config.refresh_rate_ms, 100);
        assert_eq!(config.max_width, 120);
        assert_eq!(config.max_height, 40);
    }

    #[test]
    fn test_tui_app_creation() {
        let app = TuiApp::new(TuiMode::ArgumentEditor);
        assert_eq!(app.input, "");
        assert_eq!(app.selected, 0);
        assert!(!app.should_quit);
    }

    #[test]
    fn test_tui_app_with_config() {
        let config = TuiConfig {
            mouse_enabled: true,
            ..Default::default()
        };
        let app = TuiApp::new(TuiMode::ConfigEditor).with_config(config);
        assert!(app.config.mouse_enabled);
    }

    #[test]
    fn test_tui_app_with_items() {
        let items = vec!["item1".to_string(), "item2".to_string()];
        let app = TuiApp::new(TuiMode::BatchOperations).with_items(items.clone());
        assert_eq!(app.items, items);
    }

    #[test]
    fn test_tui_app_with_input() {
        let input = "test input".to_string();
        let app = TuiApp::new(TuiMode::Install).with_input(input.clone());
        assert_eq!(app.input, input);
    }

    #[test]
    fn test_handle_resize() {
        let mut app = TuiApp::new(TuiMode::ArgumentEditor);
        app.handle_resize(100, 30);
        assert_eq!(app.terminal_size, (100, 30));
    }
}
