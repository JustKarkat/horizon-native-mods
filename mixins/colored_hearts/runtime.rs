// The colored hearts mod, formerly the game's `coloredHearts` extra: past ten
// hearts, the HUD's heart row tints one color per row of ten, health first
// then absorption with its own colors. The colors are `#RRGGBB` lists in the
// mod's data files (`healthColors`, `absorptionColors`); the tinted sprites
// are baked into the HUD atlas at every build, so turning the mod on needs no
// restart. This file is both the mod's Cargo library (its tests) and the
// module the bundle adds to the game; the game-facing helpers are the
// bundle's other `.rs` files.

/// The mod's id (its settings folder), name and version.
pub const ID: &str = "colored_hearts";
pub const NAME: &str = "Colored Hearts";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Past ten hearts, the heart row tints one color per row of ten, health first then absorption. Horizon's own.";

/// The data file keys, one file per key in the mod's data folder.
pub const KEY_COLORED: &str = "colored";
pub const KEY_HEALTH: &str = "healthColors";
pub const KEY_ABSORPTION: &str = "absorptionColors";

/// The stored `colored` value: off unless it says `true`.
pub fn read_on(stored: Option<String>) -> bool {
    stored.is_some_and(|v| v.trim() == "true")
}

/// The settings page's one button.
pub fn on_label(on: bool) -> String {
    format!("Colored: {}", if on { "ON" } else { "OFF" })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_unless_stored_true() {
        assert!(!read_on(None));
        assert!(!read_on(Some("false\n".into())));
        assert!(read_on(Some("true".into())));
        assert_eq!(on_label(true), "Colored: ON");
        assert_eq!(on_label(false), "Colored: OFF");
    }
}
