// The mod's settings page: the readout and its coordinates line.
fn official_target_details_page(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    use crate::game::official_target_details as r;
    if mod_id != r::ID {
        return;
    }
    buttons.push(r::shown_label(r::shown(|| game.native_setting(r::ID, r::KEY_SHOWN))));
    buttons.push(r::coordinates_label(r::coordinates(|| {
        game.native_setting(r::ID, r::KEY_COORDINATES)
    })));
}
