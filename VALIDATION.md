# Validation

Validated on Linux (CachyOS, Rust 1.95.0 and 1.99.1) on 2026-10-30, using the
exact revisions in `compatibility.json` (the native-health-HUD hook point's
commit, PR #4600 — repin to the merged commit when it lands), a Horizon clone
holding that commit, and the real bundle committed here.

| Check | Result |
|---|---|
| `cargo fmt --all --check` | Pass |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | Pass |
| `cargo test --workspace --locked --offline` | Runtime unit test passes |
| `python3 tools/test_bundle.py` | Manifest/pin agreement; packaging deterministic (same SHA-256 twice) |
| `python3 tools/test_stage.py` | Complete pinned Horizon export stages deterministically; original checkout unchanged; all six HEAD helpers and five support modules match the plan; every other AST pristine |
| `python3 tools/horizon.py check` | The actual `horizon-client` binary checks successfully with `--no-default-features` |
| `python3 tools/horizon.py test` | The runtime's test passes inside the game's test binary, and the 5 colored-layout tests pass in the render crate's tests |

Staging the full export took about 28 seconds per command. No game was
launched and no Minecraft resources or player data were read or changed
beyond the staging export. The on-screen rows are a manual check with the
compatible prepared resources; they carry the extras' proof (PR #3455's
120/120 HUD parity and screenshots) — same positions, same colors, same
deterministic expansion.
