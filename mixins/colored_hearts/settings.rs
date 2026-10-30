// The mod's settings page: one button, the hearts colored or not.
fn official_colored_hearts_page(mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::official_colored_hearts_mod as m;
    if mod_id != m::ID {
        return;
    }
    buttons.push(m::on_label(horizon_client_render::official_colored_hearts::colored()));
}
