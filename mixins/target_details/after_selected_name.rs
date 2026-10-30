// Runs when Horizon's selected item name has drawn: the readout sits above
// the hotbar, stacked over it.
fn after_selected_name(
    value: (),
    game: &crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    bar_height: i32,
) {
    crate::game::official_target_details_glue::draw(game, batch, bar_height);
}
