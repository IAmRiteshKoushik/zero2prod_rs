# User stories

- As a blog visitor, I want to subscribe to the newsletter, so that I can 
receive email updates when new content is published on the blog;
- As a blog author, I want to send an email to all my subscribers, so that I 
can finally notify them when new content is published
- As a subscriber, I want to be able to unsubscribe from the newsletter, so that 
I can stop receive email updates from this blog.

# Features NOT to add

- managing multiple newsletters
- segment subscribers in multiple audiences
- track opening and click rates

# Landmines (i guess)
- The book claims that `cargo add` is not a default command and you need 
`cargo-edit` for it. While the crate was useful, in 2026, `cargo add` has been 
added as a default command. Additionally, `cargo rm` and `cargo upgrade` have 
also been integrated natively.

- 

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

[link to sample github actions](https://gist.github.com/LukeMathWalker/5ae1107432ce283310c3e601fac915f3)
