// A click on the mod's settings page: show or hide the readout, add or drop
// its coordinates line, and keep both.
fn official_target_details_click(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    button: usize,
) {
    use crate::game::official_target_details as r;
    if mod_id != r::ID {
        return;
    }
    match button {
        0 => {
            let get = || game.native_setting(r::ID, r::KEY_SHOWN);
            let shown = !r::shown(get);
            r::set_shown(shown);
            game.native_set_setting(r::ID, r::KEY_SHOWN, if shown { "true" } else { "false" });
        }
        1 => {
            let get = || game.native_setting(r::ID, r::KEY_COORDINATES);
            let coordinates = !r::coordinates(get);
            r::set_coordinates(coordinates);
            game.native_set_setting(
                r::ID,
                r::KEY_COORDINATES,
                if coordinates { "true" } else { "false" },
            );
        }
        _ => {}
    }
}
