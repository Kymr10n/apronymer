# Pre-commit Hook Setup

This project uses a git pre-commit hook to enforce code quality standards.

## What it checks

1. **Code Formatting** - Runs `cargo fmt --check`
2. **Linting** - Runs `cargo clippy` with warnings as errors
3. **Tests** - Runs full test suite with `cargo test`

## Installation

The hook is already installed at `.git/hooks/pre-commit` and is executable.

## Bypassing the hook

If you need to commit without running checks (not recommended):
```bash
git commit --no-verify
```

## Manual execution

To run the same checks manually:
```bash
cd backend
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Fixing issues

- **Formatting**: Run `cargo fmt` to auto-fix
- **Clippy**: Run `cargo clippy --fix` to auto-fix where possible
- **Tests**: Fix the failing tests before committing
