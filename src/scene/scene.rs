use bevy::{prelude::*, tasks::IoTaskPool};
use std::{fs::File, io::Write};

use crate::actions::MovementPropertiesResource;

const NEW_SCENE_FILE_PATH: &str = "scenes/generated_scene.scn.ron";

pub fn save_scene_system(world: &mut World) {
    let mut builder = DynamicSceneBuilder::from_world(&world);
    builder.deny_all_resources();
    //builder.deny_all();
    builder.deny::<ComputedVisibility>();
    builder.allow_resource::<MovementPropertiesResource>();
    builder.extract_entities(world.iter_entities().map(|entity| entity.id()));
    builder.extract_resources();
    let scene = builder.build();
    let type_registry = world.resource::<AppTypeRegistry>();
    let serialized_scene = scene.serialize_ron(type_registry).unwrap();

    info!("{}", serialized_scene);

    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            dbg!(File::create(format!("assets/{NEW_SCENE_FILE_PATH}")))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}
