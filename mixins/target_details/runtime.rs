// The target details mod, formerly the game's `targetDetails` extra: a small
// readout of what the crosshair is over, centered above the hotbar — the
// looked-at block's name, or the looked-at mob's name and its health. The
// idea is Horizon's own extra's, ported to a native mod; nothing here runs
// while the mod is off. This file is both the mod's Cargo library (its
// tests) and the module the bundle adds to the game; the game-facing draw is
// glue.rs and the hooks are the bundle's other `.rs` files.

/// The mod's id (its settings folder), name and version.
pub const ID: &str = "target_details";
pub const NAME: &str = "Target Details";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "What the crosshair is over, above the hotbar: the block's name, or the mob's name and its health. Horizon's own.";

/// The data file keys, one file per key in the mod's data folder.
pub const KEY_SHOWN: &str = "shown";
pub const KEY_COORDINATES: &str = "coordinates";

/// The readout's background, like the corner panels' (`extras_hud`).
pub const BACKGROUND: u32 = 0x7800_0000;
/// The name line's color; the second line's grays.
pub const NAME_LINE: u32 = 0xFFFF_FFFF;
pub const DETAIL: u32 = 0xFFAA_AAAA;
/// The line height in GUI pixels and the box's padding.
pub const LINE: i32 = 10;
pub const PAD: i32 = 2;

/// One readout for a frame: the lines' texts with their colors, and the
/// block or mob's position when it should show its coordinates.
pub struct Inputs {
    pub w: i32,
    pub h: i32,
    /// The HUD's height above the hotbar (`bar_height`): the box sits above it.
    pub bar_height: i32,
    pub lines: Vec<(String, u32)>,
    pub at: Option<[i32; 3]>,
}

/// The health line's color: green at full health toward red at none.
pub fn health_color(health: f32, max: f32) -> u32 {
    let f = if max <= 0.0 {
        0.0
    } else {
        (health / max).clamp(0.0, 1.0)
    };
    // lerp(a, b, f); red drains as green rises, blue (minor) follows.
    let lerp = |a: u32, b: u32| (a as f32 + (b as f32 - a as f32) * f).round() as u32;
    let (r, g) = (lerp(255, 85), lerp(0, 255));
    argb(255, r, g, lerp(0, 85))
}

/// `abgr`-order color, as the HUD's draws carry it.
pub fn argb(a: u32, r: u32, g: u32, b: u32) -> u32 {
    (a << 24) | (r << 16) | (g << 8) | b
}

impl Inputs {
    /// The box's lines: the readout's own, then the coordinates line.
    fn lines(&self) -> Vec<(String, u32)> {
        let mut lines = self.lines.clone();
        if let Some([x, y, z]) = self.at {
            lines.push((format!("{x} {y} {z}"), DETAIL));
        }
        lines
    }
}

/// The readout's placement for a screen of `w` x `h` GUI pixels; `width`
/// measures the texts with the font (lines wider than the box's place are
/// left as is). The box's rect comes back first, then one entry per line.
pub struct Placed {
    pub box_rect: [i32; 4],
    pub lines: Vec<(String, u32, i32, i32)>,
}

pub fn layout(i: &Inputs, width: &dyn Fn(&str) -> i32) -> Option<Placed> {
    let lines = i.lines();
    if lines.is_empty() {
        return None;
    }
    let box_w = lines
        .iter()
        .map(|(text, _)| width(text) + 4)
        .max()
        .unwrap_or(0);
    let box_h = LINE * lines.len() as i32 + PAD;
    let x0 = i.w / 2 - box_w / 2;
    let y0 = i.h - i.bar_height - box_h - 4;
    Some(Placed {
        box_rect: [x0, y0, x0 + box_w, y0 + box_h],
        lines: lines
            .into_iter()
            .enumerate()
            .map(|(n, (text, color))| (text, color, x0 + 2, y0 + PAD + n as i32 * LINE))
            .collect(),
    })
}

/// The stored `shown` value: shown unless it says `false`.
pub fn read_shown(stored: Option<String>) -> bool {
    stored.is_none_or(|v| v.trim() != "false")
}

/// The stored `coordinates` value: off unless it says `true`.
pub fn read_coordinates(stored: Option<String>) -> bool {
    stored.is_some_and(|v| v.trim() == "true")
}

thread_local! {
    /// The settings, read once from their files, then kept.
    static SHOWN: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
    static COORDINATES: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
}

/// Whether the readout shows, read once through `get` (the `shown` file).
pub fn shown(get: impl FnOnce() -> Option<String>) -> bool {
    SHOWN.with(|s| {
        let found = s.get().unwrap_or_else(|| read_shown(get()));
        s.set(Some(found));
        found
    })
}

/// Whether the readout adds the coordinates line, read once through `get`.
pub fn coordinates(get: impl FnOnce() -> Option<String>) -> bool {
    COORDINATES.with(|c| {
        let found = c.get().unwrap_or_else(|| read_coordinates(get()));
        c.set(Some(found));
        found
    })
}

/// Replaces the cached settings (the settings page changed them).
pub fn set_shown(shown: bool) {
    SHOWN.with(|s| s.set(Some(shown)));
}
pub fn set_coordinates(coordinates: bool) {
    COORDINATES.with(|c| c.set(Some(coordinates)));
}

pub fn shown_label(shown: bool) -> String {
    format!("Shown: {}", if shown { "ON" } else { "OFF" })
}
pub fn coordinates_label(coordinates: bool) -> String {
    format!("Coordinates: {}", if coordinates { "ON" } else { "OFF" })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn width(s: &str) -> i32 {
        s.len() as i32 * 5
    }

    #[test]
    fn the_box_centers_above_the_hotbar() {
        let i = Inputs {
            w: 400,
            h: 300,
            bar_height: 49,
            lines: vec![("Stone".into(), NAME_LINE)],
            at: None,
        };
        let placed = layout(&i, &width).unwrap();
        // 5x9 text is 25 wide: box 29x12 at x (200 - 14), bottom at
        // 300 - 49 - 12 - 4 = 235.
        assert_eq!(placed.box_rect, [186, 235, 186 + 29, 235 + 12]);
        assert_eq!(placed.lines.len(), 1);
        assert_eq!(placed.lines[0], ("Stone".into(), NAME_LINE, 188, 237));
    }

    #[test]
    fn the_health_line_runs_green_to_red() {
        assert_eq!(health_color(20.0, 20.0), argb(255, 85, 255, 85));
        assert_eq!(health_color(0.0, 20.0), argb(255, 255, 0, 0));
        let half = health_color(10.0, 20.0);
        let [.., r, g, b] = half.to_be_bytes();
        assert_eq!((r, g, b), (170, 128, 43));
        // No health bar at all stays at the empty end, and overhealth clamps.
        assert_eq!(health_color(5.0, 0.0), argb(255, 255, 0, 0));
        assert_eq!(health_color(30.0, 20.0), argb(255, 85, 255, 85));
    }

    #[test]
    fn the_coordinates_line_comes_last() {
        let i = Inputs {
            w: 400,
            h: 300,
            bar_height: 49,
            lines: vec![("Stone".into(), NAME_LINE)],
            at: Some([12, -3, 70]),
        };
        let placed = layout(&i, &width).unwrap();
        assert_eq!(placed.lines.len(), 2);
        assert_eq!(placed.lines[1].0, "12 -3 70");
        assert_eq!(placed.lines[1].1, DETAIL);
    }

    #[test]
    fn nothing_without_lines() {
        let i = Inputs {
            w: 400,
            h: 300,
            bar_height: 49,
            lines: Vec::new(),
            at: None,
        };
        assert!(layout(&i, &width).is_none());
    }

    #[test]
    fn settings_read_and_label() {
        assert!(read_shown(None));
        assert!(read_shown(Some("true".into())));
        assert!(!read_shown(Some("false\n".into())));
        assert!(!read_coordinates(None));
        assert!(read_coordinates(Some("true".into())));
        assert!(!read_coordinates(Some("false".into())));
        assert_eq!(shown_label(false), "Shown: OFF");
        assert_eq!(coordinates_label(true), "Coordinates: ON");
    }
}
