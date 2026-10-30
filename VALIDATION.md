# Validation

Validated on Linux (CachyOS, Rust 1.95.0 and 1.99.1) on 2026-10-30, using the
exact revisions in `compatibility.json`, a Horizon clone holding the pin, and
the real bundle committed here.

| Check | Result |
|---|---|
| `cargo fmt --all --check` | Pass |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | Pass |
| `cargo test --workspace --locked --offline` | 5 runtime unit tests pass |
| `python3 tools/test_bundle.py` | Manifest/pin agreement; packaging deterministic (same SHA-256 twice) |
| `python3 tools/test_stage.py --horizon ../horizon` | Complete pinned Horizon export stages deterministically; original checkout unchanged; planned additions audit passes (HEAD helpers exact, RETURN helper present, modules exact, all other ASTs pristine) |
| `python3 tools/horizon.py check --horizon ../horizon` | The actual `horizon-client` binary checks successfully with `--no-default-features` (38 s with cached dependencies) |
| `python3 tools/horizon.py test --horizon ../horizon` | All 5 runtime tests pass inside the actual Horizon game test binary |

Staging the full export took about 26 seconds per command. The source audit
verifies the staged files differ from pristine only by the planned additions;
the RETURN hook's body shape is the compiler's and the embedded tests' to
check (the audit verifies the helper runs in its planned target and that no
other code changed).

No game was launched and no Minecraft resources or player data were read or
changed beyond the staging export. The on-screen readout is a manual check
with the compatible prepared resources; its layout math carries the extras'
proof (PR #3457's screenshots) — same positions, same colors.
