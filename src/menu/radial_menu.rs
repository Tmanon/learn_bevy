use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
pub struct Menu;

pub fn radial_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keys: Res<Input<KeyCode>>,
    query: Query<Entity, With<Menu>>,
) {
    if keys.just_pressed(KeyCode::M) {
        if query.is_empty() {
            // Hexagon
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::RegularPolygon::new(400.0, 6).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::rgb(0.59, 0.55, 0.45))),
                    transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                    ..default()
                },
                Menu,
            ));
        } else {
            for entity in query.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}
