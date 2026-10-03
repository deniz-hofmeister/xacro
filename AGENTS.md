# AGENTS.md

Guidance for AI agents (and new human contributors) working on this repository.
This file is normative: follow it unless the maintainer explicitly overrides it.

## What this crate is

`xacro` is a pure-Rust xacro preprocessor: it expands xacro documents into
URDF robot descriptions without a ROS, Python, or C++ installation.
Its priorities, in this order:

1. **A Rust-first library.** Idiomatic, modern Rust and its design ethics come
   first: `Result` over panics, constructors over field-poking, invariants
   enforced at the API boundary rather than promised in documentation, invalid
   states made unrepresentable — or rejected with an error where they cannot be.
2. **A safety-critical mindset.** The output describes robots: joint origins,
   axes, limits, masses. The worst failure mode is not an error and not a
   panic — it is a plausible-looking URDF that is silently wrong, because
   nothing downstream can tell it apart from a correct one. Every design
   decision is weighed against that failure mode first.
3. **Only lastly, fidelity to ROS 2 xacro.** For the in-scope feature set, a
   document must expand to the URDF upstream xacro produces — existing robot
   descriptions are the migration path into native Rust robotics. Zero desire
   to match upstream's Python API, command line, or out-of-scope features.
   Never justify a design with "that is how upstream does it"; justify it by
   the output it produces.

## Mindset: minimal and intentional

Every line must earn its place; the default answer to "should this exist?" is
no. Concretely:

- No speculative API. Nothing is added because a downstream user *might* want
  it — additions follow a demonstrated, concrete need.
- Prefer deletion. A fix that removes code beats one that adds code; treat a
  net-negative diff as evidence of a good change.
- Write the version without the new abstraction first. Keep a trait, wrapper,
  or type parameter only if the concrete version is demonstrably worse.
- One purpose per change. No "while I'm here" refactors, helpers, or options
  riding along with a fix.
- Question single-use generality: a helper with one caller, a type parameter
  with one instantiation, a config knob with one setting.

## Non-negotiables

- `#![forbid(unsafe_code)]`. No exceptions.
- Pure Rust, end to end. No Python, C++, or other language in the crate, its
  dependencies, its tests, or the repository's tooling, and no ROS toolchain
  (`rospack`, `ament`, `colcon`) at build, test, or run time. Native Rust
  robotics without those runtimes is the crate's reason to exist. A check
  that needs the upstream implementation as a reference runs outside the
  repository, is announced to the maintainer first, and leaves nothing behind.
- No new dependencies without maintainer approval. `xmltree` and `thiserror`
  are the runtime dependency list. (The `[dev-dependencies]` are expected and
  do not contradict this.)
- The README **Non-Goals** section is load-bearing. Do not implement anything
  it lists even if an issue requests it; redirect to the maintainer.
- Library code must not panic on reachable paths, and must not print: no
  `println!`, `eprintln!`, or `dbg!` outside tests. A library reports through
  its return values.

## Correctness principles

Every one of these guards against a silently wrong URDF:

- A reference that cannot be resolved — property, macro, macro parameter,
  `$(arg)`, `$(find)`, include target — is an error. Never substitute an empty
  string, a default the document did not declare, or the unexpanded text.
- An expression that cannot be evaluated exactly is an error. Never guess at
  a value, a type, or an operator's meaning.
- A xacro construct the crate does not implement is an error, never copied to
  the output verbatim or dropped. Unknown input fails loudly.
- Expansion terminates: recursion through macros and includes is bounded and
  reports an error when the bound is hit.
- Output never depends on hash-map iteration order or on anything outside the
  document, its includes, and the caller's `XacroOptions`.

When you fix a correctness bug, ship the regression test that fails on the old
code in the same commit.

## Style

The gate below machine-checks lints, formatting, and docs; everything else in
this section is convention, enforced in review — follow it anyway.

- `#![warn(missing_docs)]` and `#![warn(clippy::pedantic)]` must stay at **zero
  warnings**. Never add a new `#[allow]` to get green; fix the cause or ask.
- Construction goes through constructors everywhere — tests, examples, docs.
- Doc comments come first, then attributes (`#[cfg]`, `#[must_use]`,
  `#[inline]`). Constructors get bare `#[must_use]` — except the fallible
  ones, where `Result` already carries it and clippy's `double_must_use`
  fires; pure transforming operations get the std phrasing
  `#[must_use = "this returns the result of the operation, without modifying the original"]`.
- Rustdoc: no `# Arguments` / `# Returns` / `# Fields` sections — fold anything
  non-obvious into prose. Keep `# Errors` and `# Panics`; `# Examples` comes
  last. No hand-maintained inventories of a module's contents (rustdoc generates
  those). Doc statements must describe actual behavior, not intent; doc examples
  use `.unwrap()` and must compile (they run as doc tests).
- Errors: `Display` messages are lowercase, single-clause, no trailing period
  (Rust API guideline C-GOOD-ERR). Every variant carries a doc comment. Error
  types live in a private `mod error;` re-exported via `pub use`. Public error
  enums are `#[non_exhaustive]`.
- Tests: no logging (no `env_logger`, no `debug!`, no `println!`), `assert_eq!`/
  `assert_ne!` over `assert!(a == b)`, and behavior-descriptive snake_case
  names. Tests are deterministic and never touch paths outside the repository.
- `tests/fixtures/ros_xacro/` and the ported upstream tests are the one place
  whose expected outputs do *not* come from this crate: they are upstream's
  own fixtures and assertions (BSD-3-Clause, license kept beside them), and
  they are what catches an expansion that is consistently, plausibly wrong —
  which every self-referential assertion passes. Never regenerate an expected
  output from this crate's output, and never edit a fixture to make a test
  pass; re-derive from upstream, deliberately, or the layer stops existing.
  A ported test file's header names the upstream tests it deliberately skips
  and why.
- Strings into `String` fields: `"a".into()`. Format strings use inline
  captures: `{x}` / `{x:?}`.

## Definition of done — the verification gate

Run `tests/test_all.sh` to completion before a change is complete. It requires
an unpinned recent nightly toolchain with rustfmt and clippy. On rustup
installations:

```sh
rustup run nightly tests/test_all.sh
```

With a Nix or distro nightly on PATH, run the script directly. It checks the
build, tests, clippy, formatting, and rustdoc including docs.rs settings. It
prints `GATE PASSED` only after all checks finish. Never weaken or bypass it.

CI runs the same script. Change the gate in the script, not in the workflow.
CI additionally checks stable clippy, native x86_64/ARM64 tests, `cargo audit`,
and published-API compatibility. Stable clippy is the arbiter when it differs
from nightly, so a green local gate makes green CI likely, not guaranteed;
reproduce with `rustup run stable cargo clippy --all-targets`.

Update user-facing docs with the behavior they describe. Keep the README
brief, link to rustdoc for API contracts, and update the changelog when
relevant. Documentation drift is a bug.

## Current state

The crate is pre-1.0 and does not yet meet this file. The gaps below are known
and are closed one purpose per change; none of them is precedent, so do not
copy a pattern from the existing code that this file forbids.

- The ported upstream tests specify the target feature set; most of them fail
  until their features land. A feature is done when its ported tests pass.
- `clippy::pedantic` and `missing_docs` are not enabled yet, `cargo fmt --check`
  reports differences, and the gate does not pass.
- Library code prints (`println!` in the macro expansion) and unwraps; the
  `if`/`unless`, loop, and element processors pass their input through.
- `xml` is a runtime dependency but only the test harness uses it.
- Edition 2021 with no declared `rust-version`; transforms, its sibling crate,
  is on edition 2024 with a CI-verified MSRV.

## API stability

- Breaking changes (signatures, enum variants, trait bounds, public paths)
  need explicit maintainer sign-off per release — a past approval does not
  carry forward. Before 1.0, they land in a minor release (0.x → 0.x+1).
- Additive API (new functions, new trait impls, adding `const` or `#[must_use]`)
  is acceptable, but anything that grows the public surface deserves a note to
  the maintainer.

## Commits and disclosure

- Branch names: `bugfix/<topic>`, `feature/<topic>`, `docs/<topic>`
  (kebab-case). Release branches (`release/vX.Y.Z`) are cut by the maintainer.
- Commit messages: imperative summary line, then a body explaining *why*.
- **AI disclosure (required):** every commit authored with AI assistance must
  carry a Linux-kernel-style trailer identifying the agent and model:

  ```
  Assisted-by: <AgentName>:<model-version>
  ```

  for example `Assisted-by: Claude:claude-fable-5`. This is assistance, not
  authorship: an AI agent must never add `Signed-off-by:` (only humans can
  certify the origin of a contribution). Harness-added trailers may coexist,
  but `Assisted-by:` must be present. The human maintainer reviews and takes
  responsibility for every merged line; see the "AI-Assisted Development"
  section of the README.

## When in doubt

- Prefer a loud error over a silent guess — in code and in your own workflow.
- If a change requires weakening the gate, widening the public API, adding a
  dependency, or touching the Non-Goals, stop and ask the maintainer.
- Read the git history of the code you are changing; the commit messages
  explain why it is the way it is.

## Releasing

Releases are cut by the maintainer. Release prep is ordinary branch work:
it lands on master through the usual branch-and-merge flow before anything
is tagged, and the tag goes on master — `cargo publish` then runs from the
tagged tree. The checklist, in order:

- Finalize `CHANGELOG.md`: set the release date and repoint its comparison to
  the tag. It ships in the immutable crate package.
- Confirm the `version` in `Cargo.toml` matches the release, regenerate
  `Cargo.lock` so it records that version (any `cargo build` after the
  bump does), and bump any version pins in the README — all committed
  together: `cargo publish` refuses a dirty tree.
- Run the full verification gate (`tests/test_all.sh`).
- Run `cargo semver-checks check-release` and confirm the diff is exactly the
  changelogged one.
- `cargo publish --dry-run` and inspect the file list — nothing missing,
  nothing that should not ship.
- Merge the release-prep branch to master. If the merge is not a
  fast-forward, re-run the gate on it — the tag must point at a tree the
  gate has seen.
- Tag `vX.Y.Z` on the merge and push the tag.
- `cargo publish`.
- Create a GitHub release for the tag; mark pre-releases as such and the
  current stable release as latest.
- The `semver` CI job checks against the latest published release; its
  baseline advances automatically after publication.
