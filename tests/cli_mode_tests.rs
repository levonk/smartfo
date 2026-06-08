#[cfg(test)]
mod mode_detection_tests {
    use smartfo::config::OutputMode;

    #[test]
    fn test_agent_session_detection() {
        // Test with CLAUDE_SESSION set
        std::env::set_var("CLAUDE_SESSION", "test");
        assert!(OutputMode::detect_agent_session());
        std::env::remove_var("CLAUDE_SESSION");

        // Test with CODEX_SESSION set
        std::env::set_var("CODEX_SESSION", "test");
        assert!(OutputMode::detect_agent_session());
        std::env::remove_var("CODEX_SESSION");

        // Test with AGENT_SESSION set
        std::env::set_var("AGENT_SESSION", "test");
        assert!(OutputMode::detect_agent_session());
        std::env::remove_var("AGENT_SESSION");

        // Test with no agent session
        assert!(!OutputMode::detect_agent_session());
    }

    #[test]
    fn test_mode_resolve() {
        // Test explicit Agent mode
        assert_eq!(OutputMode::Agent.resolve(), OutputMode::Agent);

        // Test explicit Human mode
        assert_eq!(OutputMode::Human.resolve(), OutputMode::Human);

        // Test Auto mode with agent session
        std::env::set_var("CLAUDE_SESSION", "test");
        assert_eq!(OutputMode::Auto.resolve(), OutputMode::Agent);
        std::env::remove_var("CLAUDE_SESSION");

        // Test Auto mode without agent session (will depend on TTY)
        // In test environment, we can't reliably test TTY detection
        // but we can test the logic path
        let resolved = OutputMode::Auto.resolve();
        // Should be either Agent or Human depending on TTY
        assert!(matches!(resolved, OutputMode::Agent | OutputMode::Human));
    }

    #[test]
    fn test_determine_mode_cli_precedence() {
        // CLI agent flag should override everything
        let mode = OutputMode::determine_mode(false, true, OutputMode::Human);
        assert_eq!(mode, OutputMode::Agent);

        // CLI human flag should override everything
        let mode = OutputMode::determine_mode(true, false, OutputMode::Agent);
        assert_eq!(mode, OutputMode::Human);

        // No CLI flags should use config
        let mode = OutputMode::determine_mode(false, false, OutputMode::Agent);
        assert_eq!(mode, OutputMode::Agent.resolve());
    }

    #[test]
    fn test_determine_mode_env_precedence() {
        // Environment variable should override config when no CLI flags
        std::env::set_var("SMARTFO_MODE", "agent");
        let mode = OutputMode::determine_mode(false, false, OutputMode::Human);
        assert_eq!(mode, OutputMode::Agent);
        std::env::remove_var("SMARTFO_MODE");

        std::env::set_var("SMARTFO_MODE", "human");
        let mode = OutputMode::determine_mode(false, false, OutputMode::Agent);
        assert_eq!(mode, OutputMode::Human);
        std::env::remove_var("SMARTFO_MODE");

        std::env::set_var("SMARTFO_MODE", "auto");
        let mode = OutputMode::determine_mode(false, false, OutputMode::Agent);
        // Auto mode should be returned as-is (not resolved)
        assert_eq!(mode, OutputMode::Auto);
        std::env::remove_var("SMARTFO_MODE");
    }

    #[test]
    fn test_determine_mode_invalid_env() {
        // Invalid environment variable should fall back to config
        std::env::set_var("SMARTFO_MODE", "invalid");
        let mode = OutputMode::determine_mode(false, false, OutputMode::Agent);
        assert_eq!(mode, OutputMode::Agent.resolve());
        std::env::remove_var("SMARTFO_MODE");
    }

    #[test]
    fn test_determine_mode_cli_overrides_env() {
        // CLI flags should override environment variable
        std::env::set_var("SMARTFO_MODE", "human");
        let mode = OutputMode::determine_mode(false, true, OutputMode::Auto);
        assert_eq!(mode, OutputMode::Agent);
        std::env::remove_var("SMARTFO_MODE");

        std::env::set_var("SMARTFO_MODE", "agent");
        let mode = OutputMode::determine_mode(true, false, OutputMode::Auto);
        assert_eq!(mode, OutputMode::Human);
        std::env::remove_var("SMARTFO_MODE");
    }
}
