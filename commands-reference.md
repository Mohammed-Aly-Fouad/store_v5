# Command Reference

## SQLX
```bash
sqlx database create              # Create the database defined in DATABASE_URL
sqlx migrate run                  # Apply all pending migrations
cargo sqlx prepare                # Cache query metadata for offline mode (sqlx-data.json)

sqlx migrate add your_file_name   # Generate a new empty migration file
sqlx database drop                # Drop the database defined in DATABASE_URL
sqlx database create              # Recreate the database
sqlx migrate run                  # Re-apply all migrations to the fresh database
```

## GIT

### Basics
```bash
git init                          # Initialize a new git repository
git add .                         # Stage all changes in the working directory
git commit -m "Initial commit"    # Commit staged changes with a message
git status                        # Show working tree status (staged/unstaged/untracked)
```

### Git push
```bash
git push --force-with-lease origin main    # Force push, but safely: aborts if remote has new commits you haven't seen (Safest)
git push -f origin main                    # Force push, overwrites remote history unconditionally (Dangerous)
```

### Git pull (hard sync with remote)
```bash
# 1. Fetch the latest changes from the remote repository (does not modify local branch)
git fetch origin

# 2. Hard reset your current branch to match the remote branch (replace 'main' with your branch name)
#    WARNING: discards all local commits/changes on this branch
git reset --hard origin/main

# 3. Remove untracked files and directories not in the remote
git clean -fd
```

### Git log & revert
```bash
git checkout <commit-hash>          # Temporarily check out an old commit (detached HEAD state)
git checkout main                   # Return to the main branch / latest commit
#########################################################
git revert <commit-hash>            # Create a new commit that undoes the given commit, keeping history intact
git push origin main                # Push the revert commit to remote
#############################################################
git reset --hard <commit-hash>      # Rewind the branch pointer to an older commit, discarding later commits
git push origin main --force        # Force-push the rewritten history (deletes commits from remote)
#############################################
git log                             # Show full commit history
git log --oneline -n 10             # Show last 10 commits, one line each
```

## CARGO

### Rust — Cargo Project
```bash
cargo new <name>                  # Create a new binary project
cargo new <name> --lib            # Create a new library project
cargo init                        # Initialize a Cargo project in an existing directory
cargo build                       # Compile in debug mode (unoptimized, with debug info)
cargo build --release             # Compile in release mode (optimized)
cargo run                         # Build and run the debug binary
cargo run --release               # Build and run the release binary
cargo check                       # Type-check the project without producing a binary (fast)
```

### Rust — Dependencies
```bash
cargo add <crate>                 # Add a dependency to Cargo.toml
cargo add <crate>@<version>       # Add a dependency pinned to a specific version
cargo remove <crate>              # Remove a dependency from Cargo.toml
cargo update                      # Update dependencies according to Cargo.toml, refresh Cargo.lock
cargo tree                        # Print the dependency tree
```

### Cargo Watch
```bash
# Watch and re-run on file changes, ignoring the CSS file, quiet output, clear screen each run
cargo watch -c -q -i "static/css/main.css" -x "run --bin nour"

# Watch and re-run the "nour" binary, quiet + clear screen
cargo watch -c -q -x "run --bin nour"

# Watch and re-run the "main" binary, quiet + clear screen
cargo watch -q -c -x "run --bin main"

cargo clean -p package_name       # Remove build artifacts for a single package only
```

## SQLX (maintenance)
```bash
# Manually remove a specific migration record from sqlx's tracking table
# (useful when a migration needs to be re-applied or was applied incorrectly)
DELETE FROM _sqlx_migrations WHERE version = 20260825060932;
```

## TRACE
```rust
// Pretty-print a debug-formatted value to the tracing info log
tracing::info!("main_categories:\n{main_categories:#?}");
```
## Rust import
```bash
# 1. Direct path to file
use crate::domain::products::dto::ProductDTO;

# 2. Shortened path (if re-exported in products/mod.rs with `pub use dto::*;`)
use crate::domain::products::ProductDTO;

# 3. Importing multiple items from the same module
use crate::domain::products::dto::{CreateProductDTO, ProductResponseDTO};
```