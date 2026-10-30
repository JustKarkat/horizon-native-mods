// The native-health-HUD hook point's helper: the mod's glue answers when the
// mod is on; nothing set draws Horizon's own rows.
fn colored_health_hud(
    game: &mut crate::game::Game<'_>,
    inputs: &horizon_client::hud_layout::HealthInputs,
    wi: i32,
    hi: i32,
    draws: &mut Option<Vec<horizon_client::hud_layout::GuiDraw>>,
) {
    crate::game::official_colored_hearts_hud::native_rows(game, inputs, wi, hi, draws);
}
