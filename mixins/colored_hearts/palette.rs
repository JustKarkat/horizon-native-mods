// Runs before every atlas build: the mod's palette and switch, read from its
// data files, so the tinted slots are always in the atlas and turning the
// mod on needs no restart.
fn before_load() {
    use crate::game::official_colored_hearts_mod as m;
    let dir = crate::game::Game::options_path()
        .with_file_name("horizon_mods")
        .join(".horizon-mod-data")
        .join(m::ID);
    let read = |key: &str| {
        std::fs::read_to_string(dir.join(key))
            .ok()
            .map(|v| v.trim().to_owned())
    };
    horizon_client_render::official_colored_hearts::set_colored(m::read_on(read(m::KEY_COLORED)));
    let health = horizon_client_render::official_colored_hearts::parse_colors(
        &read(m::KEY_HEALTH).unwrap_or_default(),
    );
    let absorption = horizon_client_render::official_colored_hearts::parse_colors(
        &read(m::KEY_ABSORPTION).unwrap_or_default(),
    );
    horizon_client_render::official_colored_hearts::set_palette(
        horizon_client_render::official_colored_hearts::Palette::new(&health, &absorption),
    );
}
