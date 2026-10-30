// The atlas bake: appends the mod's tinted heart sprites to the packed
// images, one slot per sprite variant and configured color, so turning the
// mod on needs no restart and a resource pack's own hearts recolor too.
// Injected into hud.rs, where the `PackedImage` alias and the packed sprites
// live; runs at `atlas_sprites`' return (the bake.rs hook), before the atlas
// layout packs the slots.

pub(super) fn bake(images: &mut Vec<super::PackedImage>) {
    let palette = crate::official_colored_hearts::palette();
    for (variant, source) in crate::official_colored_hearts::VARIANT_SOURCES
        .iter()
        .enumerate()
    {
        let Some((w, h, pixels)) = images
            .iter()
            .find(|image| image.0 == *source)
            .map(|image| (image.1, image.2, image.3.clone()))
        else {
            continue;
        };
        for (absorption, colors) in [(false, &palette.health), (true, &palette.absorption)] {
            for (color, target) in colors.iter().enumerate() {
                images.push((
                    crate::official_colored_hearts::slot_atlas_name(absorption, variant, color),
                    w,
                    h,
                    crate::official_colored_hearts::recolored(&pixels, *target),
                ));
            }
        }
    }
}
