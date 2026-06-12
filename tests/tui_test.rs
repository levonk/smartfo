//! Integration tests for TUI mode

#[test]
fn test_tui_support_detection() {
    // Test that TUI support detection works
    // This test should pass in a terminal environment
    let is_supported = smartfo::tui::is_tui_supported();
    // We can't assert a specific value since it depends on the test environment
    // Just ensure the function doesn't panic
    let _ = is_supported;
}

#[test]
fn test_tui_config_default() {
    let config = smartfo::tui::TuiConfig::default();
    assert!(!config.mouse_enabled);
    assert_eq!(config.refresh_rate_ms, 100);
    assert_eq!(config.max_width, 120);
    assert_eq!(config.max_height, 40);
}

#[test]
fn test_tui_app_creation() {
    let app = smartfo::tui::TuiApp::new(smartfo::tui::TuiMode::ArgumentEditor);
    assert_eq!(app.input(), "");
    assert_eq!(app.selected(), 0);
    assert!(!app.should_quit());
}

#[test]
fn test_tui_app_with_config() {
    let config = smartfo::tui::TuiConfig {
        mouse_enabled: true,
        ..Default::default()
    };
    let app = smartfo::tui::TuiApp::new(smartfo::tui::TuiMode::ConfigEditor)
        .with_config(config);
    assert!(app.config().mouse_enabled);
}

#[test]
fn test_tui_app_with_items() {
    let items = vec!["item1".to_string(), "item2".to_string()];
    let app = smartfo::tui::TuiApp::new(smartfo::tui::TuiMode::BatchOperations)
        .with_items(items.clone());
    assert_eq!(app.items(), items.as_slice());
}

#[test]
fn test_tui_app_with_input() {
    let input = "test input".to_string();
    let app = smartfo::tui::TuiApp::new(smartfo::tui::TuiMode::Install)
        .with_input(input.clone());
    assert_eq!(app.input(), input.as_str());
}

#[test]
fn test_handle_resize() {
    let mut app = smartfo::tui::TuiApp::new(smartfo::tui::TuiMode::ArgumentEditor);
    app.handle_resize(100, 30);
    assert_eq!(app.terminal_size(), (100, 30));
}

#[test]
fn test_tui_mode_variants() {
    // Test that all TUI modes can be created
    let _ = smartfo::tui::TuiApp::new(smartfo::tui::TuiMode::ArgumentEditor);
    let _ = smartfo::tui::TuiApp::new(smartfo::tui::TuiMode::ConfigEditor);
    let _ = smartfo::tui::TuiApp::new(smartfo::tui::TuiMode::Install);
    let _ = smartfo::tui::TuiApp::new(smartfo::tui::TuiMode::BatchOperations);
}

// Note: Full integration tests that actually run the TUI event loop
// are difficult to test in automated CI environments since they require
// a terminal. These would need to be run manually or with special test
// fixtures that mock the terminal.
