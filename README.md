# Target Details — a native Horizon mod

What the crosshair is over, above the hotbar: the looked-at block's name, or
the looked-at mob's name and its health, colored green toward red. Horizon's
own `targetDetails` extra, moved out of the game as a native mod. Off by
default; the readout hides with the debug screen like the other corners, and
spectators pass by (they don't reach the hook).

## Install

Package the bundle and install it from the launcher's Mods page (or unpack the
`.hnmod` and run `horizon-launcher native-build <folder> --trust-native-code`):

```sh
python3 tools/package.py   # writes target/target_details-1.0.0.hnmod
```

The first build downloads Rust and compiles the game (20 to 30 minutes; later
builds are faster). The mod pins Horizon Nightly 2026-10-30
(`ba6344a59cb9e755b730dcf02285fc0d33b97d3c`) — see `compatibility.json`.

## Use

- **Options → Mods → Target Details → Shown** turns the readout on and off.
- **Coordinates** adds the looked-at block's or mob's position under its name.
- The settings live in the mod's data folder
  (`horizon-mods/.horizon-mod-data/target_details/`), one file per key.

## How it works

A horizon-mixins bundle (see `mixins/target_details/horizon-mixin.toml`):

- `runtime.rs` — the pure logic and settings, both the standalone Cargo
  library and the module the bundle adds to the game.
- `glue.rs` — the game-facing draw: the crosshair's target through the game's
  own `mob_hit()`/`pick()`, the lines, the paint.
- A RETURN hook on `Game::draw_selected_name` stacks the readout above the
  hotbar right after Horizon's selected item name draws.
- The usual Options → Mods hooks (`native_mods`, `native_settings`,
  `native_settings_click`) list the mod and keep its settings, as Horizon's
  own native mods do.

## Validation

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo test --workspace --locked --offline
python3 tools/test_bundle.py
# With a Horizon Git clone containing the pin:
python3 tools/test_stage.py --horizon ../horizon
python3 tools/horizon.py check --horizon ../horizon
python3 tools/horizon.py test --horizon ../horizon
```

Results on this pin: see [VALIDATION.md](VALIDATION.md). The readout's
behavior is the extras' proven behavior (the 10-29 proof on Horizon PR
#3457): the layout math is the same code, re-based on the render-crate types.
