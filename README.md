# Colored Hearts — a native Horizon mod

Past ten hearts, the HUD's heart row tints one color per row of ten — health
first, then absorption with its own colors — instead of adding rows. Horizon's
own `coloredHearts` extra, moved out of the game as a native mod. Off by
default; the colors are `#RRGGBB` lists in the mod's data files, with the
vanilla red and gold first when unset.

## Install

Package the bundle and install it from the launcher's Mods page (or unpack the
`.hnmod` and run `horizon-launcher native-build <folder> --trust-native-code`):

```sh
python3 tools/package.py   # writes target/colored_hearts-1.0.0.hnmod
```

The first build downloads Rust and compiles the game (20 to 30 minutes; later
builds are faster). The mod pins the commit that carries the
native-health-HUD hook point (`83e8e0ba`, Horizon PR #4600) — see
`compatibility.json`.

## Use

- **Options → Mods → Colored Hearts → Colored** turns the tint on and off.
- `healthColors` / `absorptionColors` in the mod's data folder
  (`horizon-mods/.horizon-mod-data/colored_hearts/`) are `#RRGGBB` lists (up
  to 32). Past the list the rows repeat it, each pass one step lighter then
  darker, growing — deterministic, so a 900-heart player climbs the same
  colors every time.
- The tinted sprites are baked into the HUD atlas at every build, so turning
  the mod on needs no restart, and a resource pack's own heart sprites
  recolor too (flat single-tone sprites are left alone).

## How it works

A horizon-mixins bundle (see `mixins/colored_hearts/horizon-mixin.toml`):

- `hearts.rs` — the recolor logic, palette and switch: a `pub` support module
  in the render crate.
- `bake_mod.rs` + a HEAD hook on `HudAtlas::slots` — one tinted slot per
  heart-sprite variant and configured color, packed with everything else.
- `layout.rs` — the answer to Horizon's `native_health_hud` hook point
  (API 14, PR #4600): Horizon's own layout drawing one colored row of ten,
  heights and food and air and armour as Horizon draws them. Nothing answered
  draws Horizon's own.
- `palette.rs` + a HEAD hook on `UiResources::load` — the current palette and
  switch before every atlas build.
- The usual Options → Mods hooks list the mod and keep its settings.

## Validation

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo test --workspace --locked --offline
python3 tools/test_bundle.py
# With a Horizon Git clone containing the pin:
python3 tools/test_stage.py --horizon ../horizon-work-hooks
python3 tools/horizon.py check --horizon ../horizon-work-hooks
python3 tools/horizon.py test --horizon ../horizon-work-hooks
```

Results: see [VALIDATION.md](VALIDATION.md). The on-screen behavior is the
extras' proven behavior (the 10-29 proof on Horizon PR #3455: 120/120 HUD
parity with the extra off, screenshots on with it); the layout code is the
same, re-based on the hook point.
