# Tips for Production

1. `cargo watch` is archived and `bacon` seems to be the new actively maintained
tool and has become the preferred standard of the Rust ecosystem.
  - It comes with better customizations
  - Cleaner terminal output
  - Hotkeys to switch between `check`, `test`, `run` and `clippy`

2. `cargo tarpaulin` is the easiest way to get started with measuring the 
code coverage for tests. Although, it is no longer the undisputed package in the 
ecosystem. A lot of projects are moving towards using `cargo-llvm-cov` as the 
modern alternative.

3. Using `cargo fmt -- --check` helps in formatting checks in the CI pipeline.

4. `cargo-audit` GitHub action along with `cargo-auditable` and `cargo-deny`
form the security layer officially maintained by the RustSec team.
