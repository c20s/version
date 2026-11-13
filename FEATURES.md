# Features

Full [SemVer](https://semver.org/) parsing, and generating the next version from the previous tag.

## Behaviour

- Input is the previous tag, e.g. `v1.2.3` or `1.2.3-beta.1+sha.abc`
- The change kind defaults to **feature** (minor bump)
- The change kind can be overridden with **fix** (patch), **feature** (minor) or **breaking** (major)
- Pre-release and build metadata are optional on the generated version
- Bumping resets lower parts: `1.2.3` + breaking = `2.0.0`, `1.2.3` + feature = `1.3.0`, `1.2.3` + fix = `1.2.4`

## API stability

- `tests/api.rs` in each package covers the public API only
- If an API test has to change, it is a breaking change and needs a major bump
- Other tests (unit tests, edge cases) can change freely

## Roadmap

### Rust package `rust/packages/version/`

- [x] Workspace and crate skeleton
- [ ] `Version` type (`major`, `minor`, `patch`)
- [ ] Parse `x.y.z` and `Display` (round-trip)
- [ ] Typed parse error
- [ ] Reject invalid input per the SemVer spec (leading zeros, missing parts, negatives)
- [ ] Pre-release identifiers
- [ ] Build metadata
- [ ] Optional `v` tag prefix when parsing
- [ ] Ordering following SemVer precedence (build metadata ignored)
- [ ] `Change` enum: `Fix`, `Feature`, `Breaking`
- [ ] `Version::next(Change)` with reset of lower parts
- [ ] Default change is `Feature`
- [ ] Optional pre-release on the next version
- [ ] Optional build metadata on the next version
- [ ] Doc examples for the public API

### Rust app `rust/apps/version/`

- [ ] CLI: previous tag in, next version out
- [ ] CLI flags for `--fix`, `--feature`, `--breaking`
- [ ] CLI flags for `--pre` and `--meta`

### TypeScript `ts/packages/version/`

- [ ] npm package wrapping the Rust package
- [ ] Same API tests as Rust, run against the npm package
