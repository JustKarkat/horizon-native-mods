// A click on the mod's settings page: color the hearts or not, and keep it.
fn official_colored_hearts_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::official_colored_hearts_mod as m;
    if mod_id != m::ID || button != 0 {
        return;
    }
    let colored = !horizon_client_render::official_colored_hearts::colored();
    horizon_client_render::official_colored_hearts::set_colored(colored);
    game.native_set_setting(m::ID, m::KEY_COLORED, if colored { "true" } else { "false" });
}
