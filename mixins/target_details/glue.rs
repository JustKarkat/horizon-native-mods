// The game-facing draw: the crosshair's target through the game's own pick,
// the readout's lines, and the paint. Injected into game.rs, where the
// Game's fields and helpers are in scope; the pure layout is the runtime
// module.

pub fn draw(
    game: &crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    bar_height: i32,
) {
    use crate::game::official_target_details as r;
    // Turned off in Options > Mods: nothing until the next build leaves it out.
    if !game.native_on(r::ID) || !r::shown(|| game.native_setting(r::ID, r::KEY_SHOWN)) {
        return;
    }
    let mut lines: Vec<(String, u32)> = Vec::new();
    // A mob before a block, as the pick order does (`pick_entity_hit`).
    let mut at = None;
    if let Some((i, _)) = game.mob_hit() {
        let m = &game.mobs[i];
        let key = format!("entity.minecraft.{}", m.kind.id);
        let name = game
            .pack
            .names
            .display_get(&key)
            .unwrap_or_else(|| m.kind.id.replace('_', " "));
        lines.push((name, r::NAME_LINE));
        lines.push((
            format!("{} / {}", m.health.ceil(), m.kind.max_health),
            r::health_color(m.health, m.kind.max_health),
        ));
        at = Some([
            m.position.x.floor() as i32,
            m.position.y.floor() as i32,
            m.position.z.floor() as i32,
        ]);
    } else if let Some(hit) = game.pick() {
        let state = game.world.block(hit.pos[0], hit.pos[1], hit.pos[2]);
        let name = game.res.facts().get(state).name.to_string();
        lines.push((game.pack.names.display_item(&name), r::NAME_LINE));
        at = Some(hit.pos);
    }
    let inputs = r::Inputs {
        w: batch.width as i32,
        h: batch.height as i32,
        bar_height,
        lines,
        at: r::coordinates(|| game.native_setting(r::ID, r::KEY_COORDINATES))
            .then_some(at)
            .flatten(),
    };
    let atlas = &game.gui.atlas;
    let width = |s: &str| horizon_client::hud::HudBatch::text_width(atlas, s) as i32;
    let Some(placed) = r::layout(&inputs, &width) else {
        return;
    };
    let [x0, y0, x1, y1] = placed.box_rect;
    let mut draws = vec![horizon_client::hud_layout::fill(
        x0, y0, x1, y1, r::BACKGROUND,
    )];
    for (text, color, x, y) in placed.lines {
        draws.push(horizon_client::hud_layout::GuiDraw::Text {
            text: horizon_client::text::Text::literal(&text),
            x,
            y,
            color,
            shadow: true,
        });
    }
    horizon_client::hud_layout::paint(batch, atlas, &draws);
}
