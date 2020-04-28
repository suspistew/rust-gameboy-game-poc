use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

const IMAGE_MISC: &str = "textures/misc.png";
const CONFIG_MISC: &str = "textures/misc.ron";

const IMAGE_CHARACTER: &str = "textures/character.png";
const CONFIG_CHARACTER: &str = "textures/character.ron";

pub fn load_misc_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_MISC, CONFIG_MISC)
}

pub fn load_character_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_CHARACTER, CONFIG_CHARACTER)
}

fn load_texture(world: &mut World, image: &str, config: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let asset_loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        asset_loader.load(
            image,
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let asset_loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    asset_loader.load(
        config,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}