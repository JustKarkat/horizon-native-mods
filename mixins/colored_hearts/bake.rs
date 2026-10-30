// Runs before the run-time slots are reserved: the mod's tinted hearts join
// the packed sprites, so the atlas layout packs them with everything else.
fn before_slots(images: &mut Vec<PackedImage>) {
    official_colored_hearts_bake::bake(images);
}
