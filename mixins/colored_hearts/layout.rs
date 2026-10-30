// The colored hearts mod's answer to the native-health-HUD hook point:
// Horizon's own `HealthHud::layout` drawing one row of ten instead of the
// rows — armour, hearts, food, air and the heights above the hotbar, as
// Horizon's layout does, with the heart rows the mod's own. Injected into
// hud_layout.rs so it can use the crate's own sprite helpers; the client-side
// glue (hud_glue.rs) calls it when the mod is on and sets nothing when it is
// off, so with the mod off the HUD is Horizon's own, byte for byte.

use horizon_core::random::{LegacyRandom, RandomSource};
use std::borrow::Cow;

use super::{ceil, sprite, Blend, GuiDraw, HealthHud, HealthInputs, HeartKind, WHITE};

/// The colored hearts (the mod's own rows): the one row of ten icons shows
/// the row of ten hearts the player is currently in, tinted that row's color
/// — health first, then absorption with its own colors. The row reads as a
/// stack: behind the current row's fill, the icons show full hearts of the
/// row below's color (in the first row, the black container, covering only a
/// half-filled icon's filled half). The rows past ten hearts are the tints
/// instead; the blink swaps the whole row to its blinking sprites.
fn colored_hearts(
    i: &HealthInputs,
    blink: bool,
    jitter: bool,
    x_left: i32,
    y_base: i32,
    next: &mut dyn FnMut(i32) -> i32,
) -> Vec<GuiDraw> {
    use crate::official_colored_hearts::MAX_ROWS;
    let at = |row: i32| (row.max(0) as usize).min(MAX_ROWS - 1);
    let mut out = Vec::new();
    let health_points = ceil(i.max_health);
    let filled = ceil(i.health) + ceil(i.absorption);
    // The row of ten the last filled half-heart sits in.
    let seg = (filled.max(1) - 1) / 20;
    let seg_start = seg * 20;
    // The row below the current one, as full hearts behind the fill: the
    // previous health or absorption color, or the last health color behind
    // the first absorption row. None in the first health row: the black
    // container shows there, as vanilla's.
    let behind = {
        let health_rows = (health_points + 19) / 20;
        if seg == 0 {
            None
        } else if seg_start >= health_points && seg - health_rows > 0 {
            // A deeper absorption row: the absorption color below it.
            Some((true, at(seg - health_rows - 1)))
        } else {
            // The first absorption row sits on the last health color; a
            // deeper health row on the color before it.
            Some((false, at(health_rows.min(seg) - 1)))
        }
    };
    for n in 0..10 {
        let x = x_left + n * 8;
        let mut y = y_base;
        if jitter {
            y += next(2);
        }
        let point = seg_start + n * 2;
        // 0 = empty, 1 = half, 2 = full.
        let fill = (filled - point).clamp(0, 2);
        let (absorption, row) = if point >= health_points {
            (true, (point - health_points) / 20)
        } else {
            (false, point / 20)
        };
        let color = at(row);
        let variant = usize::from(i.hardcore) * 4 + usize::from(fill == 1) * 2 + usize::from(blink);
        if let Some((behind_absorption, behind_color)) = behind {
            // A full heart of the row below's color.
            let behind_variant = usize::from(i.hardcore) * 4 + usize::from(blink);
            out.push(GuiDraw::Sprite {
                sprite: Cow::Owned(crate::official_colored_hearts::slot_id(
                    behind_absorption,
                    behind_variant,
                    behind_color,
                )),
                x,
                y,
                w: 9,
                h: 9,
                region: None,
                color: WHITE,
                blend: Blend::Textured,
            });
        } else {
            let container = format!(
                "hud/heart/container{}{}",
                if i.hardcore { "_hardcore" } else { "" },
                if blink { "_blinking" } else { "" }
            );
            if fill == 1 {
                // The container covers only the filled half.
                out.push(GuiDraw::Sprite {
                    sprite: Cow::Owned(container),
                    x,
                    y,
                    w: 5,
                    h: 9,
                    region: Some([9, 9, 0, 0]),
                    color: WHITE,
                    blend: Blend::Textured,
                });
            } else {
                out.push(GuiDraw::Sprite {
                    sprite: Cow::Owned(container),
                    x,
                    y,
                    w: 9,
                    h: 9,
                    region: None,
                    color: WHITE,
                    blend: Blend::Textured,
                });
            }
        }
        if fill > 0 {
            out.push(GuiDraw::Sprite {
                sprite: Cow::Owned(crate::official_colored_hearts::slot_id(
                    absorption,
                    variant,
                    color,
                )),
                x,
                y,
                w: 9,
                h: 9,
                region: None,
                color: WHITE,
                blend: Blend::Textured,
            });
        }
    }
    out
}

/// Horizon's `HealthHud::layout`, with the one colored row of ten for the
/// heart rows. The blink and display state, the heights and the bubble-pop
/// sound are this answer's to keep, as they are the layout's in Horizon.
pub fn answer(hud: &mut HealthHud, i: &HealthInputs, w: i32, h: i32) -> Vec<GuiDraw> {
    let mut out = Vec::new();
    let tick = i64::from(i.tick);
    let current = ceil(i.health);
    let blink = hud.health_blink_time > tick && (hud.health_blink_time - tick) / 3 % 2 == 1;
    if current < hud.last_health && i.damage_cooldown {
        hud.last_health_time = i.millis;
        hud.health_blink_time = tick + 20;
    } else if current > hud.last_health && i.damage_cooldown {
        hud.last_health_time = i.millis;
        hud.health_blink_time = tick + 10;
    }
    if i.millis - hud.last_health_time > 1000 {
        hud.display_health = current;
        hud.last_health_time = i.millis;
    }
    hud.last_health = current;
    let old = hud.display_health;
    let mut random = LegacyRandom::new(i64::from(i.tick.wrapping_mul(312_871)));
    let mut next = |bound: i32| random.next_int_bound(bound).unwrap_or(0);
    let (x_left, x_right, y_base) = (w / 2 - 91, w / 2 + 91, h - 39);
    let absorption = ceil(i.absorption);
    // One row of ten, so the armour row sits one row above the hotbar however
    // many hearts the player has.
    let (rows, row_height) = (1, 10);
    let mut y_air = y_base - 10;
    hud.left_height = 39 + (rows - 1) * row_height + 10 + if i.armor > 0 { 10 } else { 0 };
    // The mount's heart rows (`extractVehicleHealth`); food adds its row below.
    hud.right_height = 39 + 10 * ((i.vehicle_hearts + 9) / 10);
    let offset_index = if i.regeneration {
        i.tick % ceil(i.max_health.max(old.max(current) as f32) + 5.0)
    } else {
        -1
    };
    if i.armor > 0 {
        let y = y_base - (rows - 1) * row_height - 10;
        for n in 0..10 {
            let x = x_left + n * 8;
            if n * 2 + 1 < i.armor {
                out.push(sprite("hud/armor_full", x, y, 9, 9));
            }
            if n * 2 + 1 == i.armor {
                out.push(sprite("hud/armor_half", x, y, 9, 9));
            }
            if n * 2 + 1 > i.armor {
                out.push(sprite("hud/armor_empty", x, y, 9, 9));
            }
        }
    }
    out.extend(colored_hearts(
        i,
        blink,
        current + absorption <= 4,
        x_left,
        y_base,
        &mut next,
    ));
    if i.vehicle_hearts == 0 {
        let (empty, half, full) = if i.hunger {
            (
                "hud/food_empty_hunger",
                "hud/food_half_hunger",
                "hud/food_full_hunger",
            )
        } else {
            ("hud/food_empty", "hud/food_half", "hud/food_full")
        };
        for n in 0..10 {
            let mut y = y_base;
            if i.saturation <= 0.0 && i.tick % (i.food * 3 + 1) == 0 {
                y += next(3) - 1;
            }
            let x = x_right - n * 8 - 9;
            out.push(sprite(empty, x, y, 9, 9));
            if n * 2 + 1 < i.food {
                out.push(sprite(full, x, y, 9, 9));
            }
            if n * 2 + 1 == i.food {
                out.push(sprite(half, x, y, 9, 9));
            }
        }
        y_air -= 10;
        hud.right_height += 10;
    }
    // Hud.extractAirBubbles.
    let max_air = i.max_air.max(1);
    let air = i.air.clamp(0, max_air);
    if i.under_water || air < max_air {
        hud.right_height += 10;
        let rows = (f64::from(i.vehicle_hearts) / 10.0).ceil() as i32;
        let y = y_air - (rows - 1) * 10;
        let bubble = |offset: i32| ceil(((air + offset) * 10) as f32 / max_air as f32);
        let full = bubble(-2);
        let popping = bubble(0);
        let delay = i32::from(air != 0 && i.under_water);
        let empty = 10 - bubble(delay);
        if !i.under_water {
            hud.last_bubble_pop_sound = 0;
        }
        for b in 1..=10 {
            let x = x_right - (b - 1) * 8 - 9;
            if b <= full {
                out.push(sprite("hud/air", x, y, 9, 9));
            } else if full != popping && b == popping && i.under_water {
                out.push(sprite("hud/air_bursting", x, y, 9, 9));
                // Hud.playAirBubblePoppedSound.
                if hud.last_bubble_pop_sound != b {
                    let volume = 0.5 + 0.1 * (empty - 3 + 1).max(0) as f32;
                    let pitch = 1.0 + 0.1 * (empty - 5 + 1).max(0) as f32;
                    hud.bubble_pop_sound = Some((volume, pitch));
                    hud.last_bubble_pop_sound = b;
                }
            } else if b > 10 - empty {
                let wobble = if empty == 10 && i.tick % 2 == 0 {
                    next(2)
                } else {
                    0
                };
                out.push(sprite("hud/air_empty", x, y + wobble, 9, 9));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs() -> (HealthHud, HealthInputs) {
        (
            HealthHud::default(),
            HealthInputs {
                tick: 0,
                millis: 0,
                health: 20.0,
                max_health: 20.0,
                absorption: 0.0,
                armor: 0,
                food: 20,
                saturation: 5.0,
                kind: HeartKind::Normal,
                regeneration: false,
                hunger: false,
                hardcore: false,
                damage_cooldown: false,
                air: 300,
                max_air: 300,
                under_water: false,
                vehicle_hearts: 0,
            },
        )
    }

    fn sprites(draws: &[GuiDraw]) -> Vec<(&str, i32, i32)> {
        draws
            .iter()
            .filter_map(|d| match d {
                GuiDraw::Sprite { sprite, x, y, .. } => Some((sprite.as_ref(), *x, *y)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn colored_hearts_show_one_row_of_ten() {
        let (mut hud, mut i) = inputs();
        i.max_health = 40.0;
        i.health = 30.0;
        let draws = answer(&mut hud, &i, 400, 300);
        let s = sprites(&draws);
        // One row of ten (y_base = 261): the second health row's color on top
        // of full hearts of the first row's color, no black containers.
        assert!(
            !s.iter()
                .any(|(n, _, _)| n.starts_with("hud/heart/container"))
        );
        let behind: Vec<_> = s
            .iter()
            .filter(|(n, _, _)| *n == "horizon/heart0x0")
            .collect();
        assert_eq!(behind.len(), 10, "the first row of health behind, full");
        let fills: Vec<_> = s
            .iter()
            .filter(|(n, _, _)| *n == "horizon/heart0x1")
            .collect();
        assert_eq!(fills.len(), 5, "the second row's color on top");
        // One row: every heart on the same y, starting at the left edge.
        assert!(s.iter().all(|(_, _, y)| *y == 261));
        assert_eq!(s.iter().map(|(_, x, _)| *x).min(), Some(109));
        // The heights above the hotbar follow the one row, not the 40 HP.
        assert_eq!(hud.left_height, 49);
    }

    #[test]
    fn absorption_tints_with_its_own_colors() {
        let (mut hud, mut i) = inputs();
        i.max_health = 20.0;
        i.health = 20.0;
        i.absorption = 8.0;
        let draws = answer(&mut hud, &i, 400, 300);
        let tinted: Vec<_> = sprites(&draws)
            .into_iter()
            .filter(|(n, _, _)| n.starts_with("horizon/"))
            .collect();
        // Four gold absorption hearts on full hearts of the last health
        // color, in the absorption list's first color.
        assert_eq!(
            tinted
                .iter()
                .filter(|(n, _, _)| *n == "horizon/aheart0x0")
                .count(),
            4
        );
        assert_eq!(
            tinted
                .iter()
                .filter(|(n, _, _)| *n == "horizon/heart0x0")
                .count(),
            10,
            "the full health's red behind"
        );
    }

    #[test]
    fn a_half_heart_container_covers_only_its_filled_half() {
        let (mut hud, mut i) = inputs();
        i.health = 1.0;
        let draws = answer(&mut hud, &i, 400, 300);
        let containers: Vec<_> = draws
            .iter()
            .filter_map(|d| match d {
                GuiDraw::Sprite {
                    sprite, region, w, ..
                } => Some((sprite.as_ref(), *region, *w)),
                _ => None,
            })
            .filter(|(n, _, _)| n.starts_with("hud/heart/container"))
            .collect();
        assert_eq!(containers.len(), 10);
        assert_eq!(
            containers[0],
            ("hud/heart/container", Some([9, 9, 0, 0]), 5)
        );
        assert!(
            containers[1..]
                .iter()
                .all(|(n, region, w)| region.is_none() && *w == 9 && *n == "hud/heart/container")
        );
    }

    #[test]
    fn rows_past_the_colors_get_expanded_ones() {
        let (mut hud, mut i) = inputs();
        i.max_health = 80.0;
        i.health = 70.0;
        let draws = answer(&mut hud, &i, 400, 300);
        // The fourth row of health draws the fourth baked color (the list
        // repeats with lighter and darker passes past its length).
        assert!(
            sprites(&draws)
                .iter()
                .any(|(n, _, _)| *n == "horizon/heart0x3")
        );
    }

    #[test]
    fn food_and_armor_draw_as_horizon_does() {
        let (mut hud, mut i) = inputs();
        i.max_health = 40.0;
        i.health = 30.0;
        i.armor = 6;
        let draws = answer(&mut hud, &i, 400, 300);
        let s = sprites(&draws);
        // Three full armour icons on the one-row height (y_base - 10 = 251),
        // and the food row at the hotbar's height.
        assert_eq!(
            s.iter()
                .filter(|(n, _, y)| *n == "hud/armor_full" && *y == 251)
                .count(),
            3
        );
        assert_eq!(
            s.iter()
                .filter(|(n, _, y)| *n == "hud/food_empty" && *y == 261)
                .count(),
            10
        );
        assert_eq!(
            s.iter()
                .filter(|(n, _, y)| *n == "hud/food_full" && *y == 261)
                .count(),
            10
        );
    }
}
