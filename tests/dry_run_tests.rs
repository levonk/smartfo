//! Dry-run mode tests

use smartfo::dry_run::DryRunContext;

#[test]
fn test_dry_run_context_default() {
    let ctx = DryRunContext::default();
    assert!(!ctx.is_enabled());
}

#[test]
fn test_dry_run_context_disabled() {
    let ctx = DryRunContext::disabled();
    assert!(!ctx.is_enabled());
}

#[test]
fn test_dry_run_context_enabled() {
    let ctx = DryRunContext::enabled();
    assert!(ctx.is_enabled());
}

#[test]
fn test_dry_run_context_new() {
    let ctx = DryRunContext::new(true);
    assert!(ctx.is_enabled());
    
    let ctx = DryRunContext::new(false);
    assert!(!ctx.is_enabled());
}

#[test]
fn test_dry_run_context_copy() {
    let ctx1 = DryRunContext::enabled();
    let ctx2 = ctx1;
    assert!(ctx2.is_enabled());
}
