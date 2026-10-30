// The game-side glue the hook helper calls: the mod's rows when it is on,
// nothing set when it is off, so Horizon's own draw.

pub fn native_rows(
    game: &mut crate::game::Game<'_>,
    inputs: &horizon_client::hud_layout::HealthInputs,
    wi: i32,
    hi: i32,
    draws: &mut Option<Vec<horizon_client::hud_layout::GuiDraw>>,
) {
    use crate::game::official_colored_hearts_mod as m;
    if !game.native_on(m::ID) {
        return;
    }
    if !horizon_client_render::official_colored_hearts::colored() {
        return;
    }
    let rows = horizon_client::hud_layout::official_colored_hearts_layout::answer(
        &mut game.health_hud,
        inputs,
        wi,
        hi,
    );
    *draws = Some(rows);
}
