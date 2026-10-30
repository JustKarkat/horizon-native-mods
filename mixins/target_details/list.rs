// The mod's row in Options > Mods.
fn official_target_details_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::official_target_details as r;
    mods.push(crate::game::native_hooks::NativeMod {
        id: r::ID.into(),
        title: format!("{} {}", r::NAME, r::VERSION),
        description: r::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
