# Features

Full [SemVer](https://semver.org/) parsing, and generating the next version from the previous tag.

## Behaviour

- Input is the previous tag, e.g. `v1.2.3` or `1.2.3-beta.1+sha.abc`
- Output never has a `v` prefix: `v1.2.3` in, `1.3.0` out
- Input may omit minor and patch, they default to `0`: `1` is `1.0.0`, `1.2` is `1.2.0`
- Output is always the full `major.minor.patch`
- The change kind defaults to **feature** (minor bump)
- The change kind can be overridden with **fix** (patch), **feature** (minor) or **breaking** (major)
- Pre-release and build metadata are optional on the generated version
- Bumping resets lower parts: `1.2.3` + breaking = `2.0.0`, `1.2.3` + feature = `1.3.0`, `1.2.3` + fix = `1.2.4`

### Initial development (`0.y.z`)

Per SemVer, `0.y.z` is not a stable API and anything may change. We use the minor part to signal breaking changes:

| Previous | Change   | Next    |
| -------- | -------- | ------- |
| `0.3.1`  | breaking | `0.4.0` |
| `0.3.1`  | feature  | `0.3.2` |
| `0.3.1`  | fix      | `0.3.2` |

- Breaking bumps minor, feature and fix both bump patch
- A bump never takes a version from `0.y.z` to `1.0.0`, the move to `1.0.0` is a deliberate release

### Pre-release

The requested pre-release is either a bare label (`beta`) or explicit (`beta.5`, `alpha.1`).

| Previous      | Requested | Next          | Rule                                                      |
| ------------- | --------- | ------------- | --------------------------------------------------------- |
| `1.3.0-beta.1`| `beta`    | `1.3.0-beta.2`| Same label, bump the pre-release number, core is kept     |
| `1.3.0-beta.1`| `beta.5`  | `1.3.0-beta.5`| Explicit, used exactly as given, takes precedence         |
| `1.3.0-beta.1`| `alpha`   | `1.3.0-alpha.1`| Different label, starts at `.1`                          |
| `1.3.0-beta.1`| `alpha.2` | `1.3.0-alpha.2`| Explicit, used exactly as given, takes precedence        |
| `1.2.3`       | `beta`    | `1.3.0-beta.1`| Stable previous, change is applied first, then `beta.1`   |
| `1.3.0-beta.1`| none      | `1.3.0`       | Release, the pre-release is dropped                       |
| `1.3.0-beta.1`| none + breaking | `2.0.0` | Release with explicit change, applied to the core as normal |
| `1.3.0-beta.1`| none + feature  | `1.4.0` | Same, applied to the core `1.3.0`                       |
| `1.3.0-beta.1`| none + fix      | `1.3.1` | Same, applied to the core `1.3.0`                       |

- The core version is locked while on a pre-release: when the previous tag is a pre-release and a pre-release is requested, an explicit change (`--fix`, `--feature`, `--breaking`) is an error, e.g. `1.3.0-beta.1` + `beta` + `--breaking` throws
- The lock only applies while staying on a pre-release. Releasing from a pre-release (no pre-release requested) with an explicit change applies that change to the core version as normal and drops the pre-release
- Only an explicit change throws, the implicit default (feature) is simply not applied
- This means the change is optional in the API (`Option<Change>`), so an explicit `feature` can be told apart from the default
- A bare label never goes backwards on its own, an explicit pre-release is trusted even if it sorts lower
- Build metadata (`+meta`) is never carried over from the previous tag, it is only set when requested

## API stability

- `tests/api.rs` in each package covers the public API only
- If an API test has to change, it is a breaking change and needs a major bump
- Other tests (unit tests, edge cases) can change freely

## Roadmap

### Rust package `rust/packages/version/`

- [x] Workspace and crate skeleton
- [x] `Version` type (`major`, `minor`, `patch`)
- [x] Parse `x.y.z` and `Display` (round-trip)
- [ ] Typed parse error
- [x] Reject invalid input per the SemVer spec (leading zeros, negatives, empty parts, more than three parts)
- [ ] Lenient parsing: missing minor and patch default to `0` (`1` is `1.0.0`, `1.2` is `1.2.0`)
- [ ] Pre-release identifiers
- [ ] Build metadata
- [ ] Optional `v` tag prefix when parsing
- [ ] Output never includes the `v` prefix
- [ ] Ordering following SemVer precedence (build metadata ignored)
- [ ] `Change` enum: `Fix`, `Feature`, `Breaking`
- [ ] `Version::next(Change)` with reset of lower parts
- [ ] Default change is `Feature`
- [ ] Change is optional (`Option<Change>`) to tell explicit from default
- [ ] Explicit change on a pre-release tag with a pre-release requested returns an error (core is locked)
- [ ] `0.y.z` rules: breaking bumps minor, feature and fix bump patch
- [ ] Explicit release of `1.0.0` (not reachable through `next`)
- [ ] Optional pre-release on the next version
- [ ] Bare label bumps the pre-release number when the label matches the previous tag
- [ ] Bare label with a different label starts at `.1`
- [ ] Explicit pre-release (`beta.5`) takes precedence over everything
- [ ] No pre-release requested on a pre-release tag releases the stable version
- [ ] No pre-release requested on a pre-release tag with an explicit change applies the change to the core (`1.3.0-beta.1` + breaking = `2.0.0`)
- [ ] Optional build metadata on the next version
- [ ] Doc examples for the public API

### Rust app `rust/apps/version/`

- [ ] CLI: previous tag in, next version out
- [ ] CLI flags for `--fix`, `--feature`, `--breaking`
- [ ] CLI flags for `--pre` and `--meta`

### TypeScript `ts/packages/version/`

- [ ] npm package wrapping the Rust package
- [ ] Same API tests as Rust, run against the npm package
