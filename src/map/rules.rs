use crate::map::assets::SpawnableAsset;
use crate::map::models::TerrainModelBuilder;
use crate::map::sockets::*;
use bevy_procedural_tilemaps::prelude::*;
use crate::map::tilemap::SpriteType;

fn build_dirt_layer(
    terrain_model_builder: &mut TerrainModelBuilder,
    terrain_sockets: &TerrainSockets,
    socket_collection: &mut SocketCollection
) {
    terrain_model_builder
        .create_model(
            SocketsCartesian3D::Simple {
                x_pos: terrain_sockets.dirt.material,
                x_neg: terrain_sockets.dirt.material,
                z_pos: terrain_sockets.dirt.layer_up,
                z_neg: terrain_sockets.dirt.layer_down,
                y_pos: terrain_sockets.dirt.material,
                y_neg: terrain_sockets.dirt.material,
            },
            vec![SpawnableAsset::new(SpriteType::Dirt)]
        )
        .with_weight(20.);

    socket_collection.add_connections(vec![(
        terrain_sockets.dirt.material,
        vec![terrain_sockets.dirt.material]
    )]);
}

fn build_grass_layer(
    terrain_model_builder: &mut TerrainModelBuilder,
    terrain_sockets: &TerrainSockets,
    socket_collection: &mut SocketCollection
) {
    terrain_model_builder.create_model(
        SocketsCartesian3D::Simple {
            x_pos: terrain_sockets.void,
            x_neg: terrain_sockets.void,
            z_pos: terrain_sockets.grass.layer_up,
            z_neg: terrain_sockets.grass.layer_down,
            y_pos: terrain_sockets.void,
            y_neg: terrain_sockets.void,
        },
        Vec::new()
    );

    terrain_model_builder.create_model(
        SocketsCartesian3D::Multiple {
            x_pos: vec![terrain_sockets.grass.material],
            x_neg: vec![terrain_sockets.grass.material],
            z_pos: vec![
                terrain_sockets.grass.layer_up,
                terrain_sockets.grass.grass_fill_up,
            ],
            z_neg: vec![terrain_sockets.grass.layer_down],
            y_pos: vec![terrain_sockets.grass.material],
            y_neg: vec![terrain_sockets.grass.material]
        },
        vec![SpawnableAsset::new(SpriteType::GreenGrass)]
    )
    .with_weight(5.);


    let green_grass_corner_out = SocketsCartesian3D::Simple {
        x_pos: terrain_sockets.grass.void_and_grass,
        x_neg: terrain_sockets.void,
        z_pos: terrain_sockets.grass.layer_up,
        z_neg: terrain_sockets.grass.layer_down,
        y_pos: terrain_sockets.void,
        y_neg: terrain_sockets.grass.grass_and_void,
    }
    .to_template();

    let green_grass_corner_in = SocketsCartesian3D::Simple {
        x_pos: terrain_sockets.grass.grass_and_void,
        x_neg: terrain_sockets.grass.material,
        z_pos: terrain_sockets.grass.layer_up,
        z_neg: terrain_sockets.grass.layer_down,
        y_pos: terrain_sockets.grass.material,
        y_neg: terrain_sockets.grass.void_and_grass,
    }
    .to_template();

    let green_grass_side = SocketsCartesian3D::Simple {
        x_pos: terrain_sockets.grass.void_and_grass,
        x_neg: terrain_sockets.grass.grass_and_void,
        z_pos: terrain_sockets.grass.layer_up,
        z_neg: terrain_sockets.grass.layer_down,
        y_pos: terrain_sockets.void,
        y_neg: terrain_sockets.grass.material,
    }
    .to_template();

    terrain_model_builder.create_model(
        green_grass_corner_out.clone(),
        vec![SpawnableAsset::new(SpriteType::GreenGrassCornerOutTopLeft)]
    );

    terrain_model_builder.create_model(
        green_grass_corner_out.rotated(ModelRotation::Rot90, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::GreenGrassCornerOutBottomLeft)]
    );

    terrain_model_builder.create_model(
        green_grass_corner_out.rotated(ModelRotation::Rot180, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::GreenGrassCornerOutBottomRight)]
    );

    terrain_model_builder.create_model(
        green_grass_corner_out.rotated(ModelRotation::Rot270, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::GreenGrassCornerOutTopRight)]
    );

    terrain_model_builder.create_model(
        green_grass_corner_in.clone(),
        vec![SpawnableAsset::new(SpriteType::GreenGrassCornerInTopLeft)]
    );

    terrain_model_builder.create_model(
        green_grass_corner_in.rotated(ModelRotation::Rot90, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::GreenGrassCornerInBottomLeft)]
    );

    terrain_model_builder.create_model(
        green_grass_corner_in.rotated(ModelRotation::Rot180, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::GreenGrassCornerInBottomRight)]
    );

    terrain_model_builder.create_model(
        green_grass_corner_in.rotated(ModelRotation::Rot270, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::GreenGrassCornerInTopRight)]
    );

    terrain_model_builder.create_model(
        green_grass_side.clone(),
        vec![SpawnableAsset::new(SpriteType::GreenGrassSideTop)]
    );

    terrain_model_builder.create_model(
        green_grass_side.rotated(ModelRotation::Rot90, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::GreenGrassSideLeft)]
    );

    terrain_model_builder.create_model(
        green_grass_side.rotated(ModelRotation::Rot180, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::GreenGrassSideBottom)]
    );

    terrain_model_builder.create_model(
        green_grass_side.rotated(ModelRotation::Rot270, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::GreenGrassSideRight)]
    );

    socket_collection.add_rotated_connection(
        terrain_sockets.dirt.layer_up,
        vec![terrain_sockets.grass.layer_down],
    );

    socket_collection.add_connections(vec![
        (terrain_sockets.void, vec![terrain_sockets.void]),
        (
            terrain_sockets.grass.material,
            vec![terrain_sockets.grass.material]
        ),
        (
            terrain_sockets.grass.void_and_grass,
            vec![terrain_sockets.grass.grass_and_void]
        ),
    ]);
}

fn build_yellow_grass_layer(
    terrain_model_builder: &mut TerrainModelBuilder,
    terrain_sockets: &TerrainSockets,
    socket_collection: &mut SocketCollection
) {
    terrain_model_builder.create_model(
        SocketsCartesian3D::Simple {
            x_pos: terrain_sockets.void,
            x_neg: terrain_sockets.void,
            z_pos: terrain_sockets.yellow_grass.layer_up,
            z_neg: terrain_sockets.yellow_grass.layer_down,
            y_pos: terrain_sockets.void,
            y_neg: terrain_sockets.void,
        },
        Vec::new()
    );

    terrain_model_builder.create_model(
        SocketsCartesian3D::Simple {
            x_pos: terrain_sockets.grass.material,
            x_neg: terrain_sockets.grass.material,
            z_pos: terrain_sockets.yellow_grass.layer_up,
            z_neg: terrain_sockets.yellow_grass.yellow_grass_fill_down,
            y_pos: terrain_sockets.grass.material,
            y_neg: terrain_sockets.grass.material,
        },
        vec![SpawnableAsset::new(SpriteType::YellowGrass)]
    )
    .with_weight(5.);

    let yellow_grass_corner_out = SocketsCartesian3D::Simple {
        x_pos: terrain_sockets.grass.void_and_grass,
        x_neg: terrain_sockets.void,
        z_pos: terrain_sockets.yellow_grass.layer_up,
        z_neg: terrain_sockets.yellow_grass.yellow_grass_fill_down,
        y_pos: terrain_sockets.void,
        y_neg: terrain_sockets.grass.grass_and_void,

    }
    .to_template();

    let yellow_grass_corner_in = SocketsCartesian3D::Simple {
        x_pos: terrain_sockets.grass.grass_and_void,
        x_neg: terrain_sockets.grass.material,
        z_pos: terrain_sockets.yellow_grass.layer_up,
        z_neg: terrain_sockets.yellow_grass.yellow_grass_fill_down,
        y_pos: terrain_sockets.grass.material,
        y_neg: terrain_sockets.grass.void_and_grass,
    }
    .to_template();

    let yellow_grass_side = SocketsCartesian3D::Simple {
        x_pos: terrain_sockets.grass.void_and_grass,
        x_neg: terrain_sockets.grass.grass_and_void,
        z_pos: terrain_sockets.yellow_grass.layer_up,
        z_neg: terrain_sockets.yellow_grass.yellow_grass_fill_down,
        y_pos: terrain_sockets.void,
        y_neg: terrain_sockets.grass.material,
    }
    .to_template();

    terrain_model_builder.create_model(
        yellow_grass_corner_out.clone(),
        vec![SpawnableAsset::new(SpriteType::YellowGrassCornerOutTopLeft)]
    );

    terrain_model_builder.create_model(
        yellow_grass_corner_out.rotated(ModelRotation::Rot90, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::YellowGrassCornerOutBottomLeft)]
    );

    terrain_model_builder.create_model(
        yellow_grass_corner_out.rotated(ModelRotation::Rot180, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::YellowGrassCornerOutBottomRight)]
    );

    terrain_model_builder.create_model(
        yellow_grass_corner_out.rotated(ModelRotation::Rot270, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::YellowGrassCornerOutTopRight)]
    );

    terrain_model_builder.create_model(
        yellow_grass_corner_in.clone(),
        vec![SpawnableAsset::new(SpriteType::YellowGrassCornerInTopLeft)]
    );

    terrain_model_builder.create_model(
        yellow_grass_corner_in.rotated(ModelRotation::Rot90, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::YellowGrassCornerInBottomLeft)]
    );

    terrain_model_builder.create_model(
        yellow_grass_corner_in.rotated(ModelRotation::Rot180, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::YellowGrassCornerInBottomRight)]
    );

    terrain_model_builder.create_model(
        yellow_grass_corner_in.rotated(ModelRotation::Rot270, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::YellowGrassCornerInTopRight)]
    );

    terrain_model_builder.create_model(
        yellow_grass_side.clone(),
        vec![SpawnableAsset::new(SpriteType::YellowGrassSideTop)]
    );

    terrain_model_builder.create_model(
        yellow_grass_side.rotated(ModelRotation::Rot90, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::YellowGrassSideLeft)]
    );

    terrain_model_builder.create_model(
        yellow_grass_side.rotated(ModelRotation::Rot180, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::YellowGrassSideBottom)]
    );

    terrain_model_builder.create_model(
        yellow_grass_side.rotated(ModelRotation::Rot270, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::YellowGrassSideRight)]
    );

    socket_collection
        .add_rotated_connection(
            terrain_sockets.grass.layer_up,
            vec![terrain_sockets.yellow_grass.layer_down]
        )
        .add_rotated_connection(
            terrain_sockets.yellow_grass.yellow_grass_fill_down,
            vec![terrain_sockets.grass.grass_fill_up]
        );
}

fn build_water_layer(
    terrain_model_builder: &mut TerrainModelBuilder,
    terrain_sockets: &TerrainSockets,
    socket_collection: &mut SocketCollection
) {
    terrain_model_builder.create_model(
        SocketsCartesian3D::Multiple {
            x_pos: vec![terrain_sockets.void],
            x_neg: vec![terrain_sockets.void],
            z_pos: vec![
                terrain_sockets.water.layer_up,
                terrain_sockets.water.ground_up,
            ],
            z_neg: vec![terrain_sockets.water.layer_down],
            y_pos: vec![terrain_sockets.void],
            y_neg: vec![terrain_sockets.void]
        },
        Vec::new()
    );

    const WATER_WEIGHT: f32 = 0.02;
    terrain_model_builder.create_model(
        SocketsCartesian3D::Simple {
            x_pos: terrain_sockets.water.material,
            x_neg: terrain_sockets.water.material,
            z_pos: terrain_sockets.water.layer_up,
            z_neg: terrain_sockets.water.layer_down,
            y_pos: terrain_sockets.water.material,
            y_neg: terrain_sockets.water.material,
        },
        vec![SpawnableAsset::new(SpriteType::Water)]
    )
    .with_weight(10. * WATER_WEIGHT);

    let water_corner_out = SocketsCartesian3D::Simple {
        x_pos: terrain_sockets.water.void_and_water,
        x_neg: terrain_sockets.void,
        z_pos: terrain_sockets.water.layer_up,
        z_neg: terrain_sockets.water.layer_down,
        y_pos: terrain_sockets.void,
        y_neg: terrain_sockets.water.water_and_void,
    }
    .to_template()
    .with_weight(WATER_WEIGHT);

    let water_corner_in = SocketsCartesian3D::Simple {
        x_pos: terrain_sockets.water.water_and_void,
        x_neg: terrain_sockets.water.material,
        z_pos: terrain_sockets.water.layer_up,
        z_neg: terrain_sockets.water.layer_down,
        y_pos: terrain_sockets.water.material,
        y_neg: terrain_sockets.water.void_and_water,
    }
    .to_template()
    .with_weight(WATER_WEIGHT);

    let water_side = SocketsCartesian3D::Simple {
        x_pos: terrain_sockets.water.void_and_water,
        x_neg: terrain_sockets.water.water_and_void,
        z_pos: terrain_sockets.water.layer_up,
        z_neg: terrain_sockets.water.layer_down,
        y_pos: terrain_sockets.void,
        y_neg: terrain_sockets.water.material,
    }
    .to_template()
    .with_weight(WATER_WEIGHT);

    terrain_model_builder.create_model(
        water_corner_out.clone(),
        vec![SpawnableAsset::new(SpriteType::WaterCornerOutTopLeft)]
    );

    terrain_model_builder.create_model(
        water_corner_out.rotated(ModelRotation::Rot90, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::WaterCornerOutBottomLeft)]
    );

    terrain_model_builder.create_model(
        water_corner_out.rotated(ModelRotation::Rot180, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::WaterCornerOutBottomRight)]
    );

    terrain_model_builder.create_model(
        water_corner_out.rotated(ModelRotation::Rot270, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::WaterCornerOutTopRight)]
    );

    terrain_model_builder.create_model(
        water_corner_in.clone(),
        vec![SpawnableAsset::new(SpriteType::WaterCornerInTopLeft)]
    );

    terrain_model_builder.create_model(
        water_corner_in.rotated(ModelRotation::Rot90, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::WaterCornerInBottomLeft)]
    );

    terrain_model_builder.create_model(
        water_corner_in.rotated(ModelRotation::Rot180, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::WaterCornerInBottomRight)]
    );

    terrain_model_builder.create_model(
        water_corner_in.rotated(ModelRotation::Rot270, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::WaterCornerInTopRight)]
    );

    terrain_model_builder.create_model(
        water_side.clone(),
        vec![SpawnableAsset::new(SpriteType::WaterSideTop)]
    );

    terrain_model_builder.create_model(
        water_side.rotated(ModelRotation::Rot90, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::WaterSideLeft)]
    );

    terrain_model_builder.create_model(
        water_side.rotated(ModelRotation::Rot180, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::WaterSideBottom)]
    );

    terrain_model_builder.create_model(
        water_side.rotated(ModelRotation::Rot270, Direction::ZForward),
        vec![SpawnableAsset::new(SpriteType::WaterSideRight)]
    );

    socket_collection.add_connections(vec![
        (
            terrain_sockets.water.material,
            vec![terrain_sockets.water.material]
        ),
        (
            terrain_sockets.water.water_and_void,
            vec![terrain_sockets.water.void_and_water]
        )
    ]);

    socket_collection.add_rotated_connection(
        terrain_sockets.yellow_grass.layer_up,
        vec![terrain_sockets.water.layer_down]
    );
}
fn build_props_layer(
    terrain_model_builder: &mut TerrainModelBuilder,
    terrain_sockets: &TerrainSockets,
    socket_collection: &mut SocketCollection
) {
    terrain_model_builder.create_model(
        SocketsCartesian3D::Multiple {
            x_pos: vec![terrain_sockets.void],
            x_neg: vec![terrain_sockets.void],
            z_pos: vec![terrain_sockets.props.layer_up],
            z_neg: vec![terrain_sockets.props.layer_down],
            y_pos: vec![terrain_sockets.void],
            y_neg: vec![terrain_sockets.void],
        },
        Vec::new()
    );

    const PROPS_WEIGHT: f32 = 0.025;
    const ROCKS_WEIGHT: f32 = 0.008;
    const PLANTS_WEIGHT: f32 = 0.025;
    const STUMPS_WEIGHT: f32 = 0.012;

    let prop = SocketsCartesian3D::Simple {
            x_pos: terrain_sockets.void,
            x_neg: terrain_sockets.void,
            z_pos: terrain_sockets.props.layer_up,
            z_neg: terrain_sockets.props.props_down,
            y_pos: terrain_sockets.void,
            y_neg: terrain_sockets.void,
    }
    .to_template()
    .with_weight(PROPS_WEIGHT);

    let plant_prop = prop.clone().with_weight(PLANTS_WEIGHT);
    let stump_prop = prop.clone().with_weight(STUMPS_WEIGHT);
    let rock_prop = prop.clone().with_weight(ROCKS_WEIGHT);

    terrain_model_builder.create_model(
        plant_prop.clone(),
        vec![
            SpawnableAsset::new(SpriteType::SmallTreeBottom),
            SpawnableAsset::new(SpriteType::SmallTreeTop).with_grid_offset(GridDelta::new(0, 1, 0))
        ]
    );

    terrain_model_builder
        .create_model(
            SocketsCartesian3D::Simple {
                x_pos: terrain_sockets.props.big_tree_1_base,
                x_neg: terrain_sockets.void,
                z_pos: terrain_sockets.props.layer_up,
                z_neg: terrain_sockets.props.props_down,
                y_pos: terrain_sockets.void,
                y_neg: terrain_sockets.void,
            },
            vec![
                SpawnableAsset::new(SpriteType::BigTree1BottomLeft),
                SpawnableAsset::new(SpriteType::BigTree1TopLeft).with_grid_offset(GridDelta::new(0, 1, 0))
            ]

        )
        .with_weight(PROPS_WEIGHT);

    terrain_model_builder
        .create_model(
            SocketsCartesian3D::Simple {
                x_pos: terrain_sockets.void,
                x_neg: terrain_sockets.props.big_tree_1_base,
                z_pos: terrain_sockets.props.layer_up,
                z_neg: terrain_sockets.props.props_down,
                y_pos: terrain_sockets.void,
                y_neg: terrain_sockets.void,
            },
            vec![
                SpawnableAsset::new(SpriteType::BigTree1BottomRight),
                SpawnableAsset::new(SpriteType::BigTree1TopRight).with_grid_offset(GridDelta::new(0, 1, 0))
            ]

        )
        .with_weight(PROPS_WEIGHT);

    terrain_model_builder
        .create_model(
            SocketsCartesian3D::Simple {
                x_pos: terrain_sockets.props.big_tree_2_base,
                x_neg: terrain_sockets.void,
                z_pos: terrain_sockets.props.layer_up,
                z_neg: terrain_sockets.props.props_down,
                y_pos: terrain_sockets.void,
                y_neg: terrain_sockets.void,
            },
            vec![
                SpawnableAsset::new(SpriteType::BigTree2BottomLeft),
                SpawnableAsset::new(SpriteType::BigTree2TopLeft).with_grid_offset(GridDelta::new(0, 1, 0))
            ]

        )
        .with_weight(PROPS_WEIGHT);

    terrain_model_builder
        .create_model(
            SocketsCartesian3D::Simple {
                x_pos: terrain_sockets.void,
                x_neg: terrain_sockets.props.big_tree_2_base,
                z_pos: terrain_sockets.props.layer_up,
                z_neg: terrain_sockets.props.props_down,
                y_pos: terrain_sockets.void,
                y_neg: terrain_sockets.void,
            },
            vec![
                SpawnableAsset::new(SpriteType::BigTree2BottomRight),
                SpawnableAsset::new(SpriteType::BigTree2TopRight).with_grid_offset(GridDelta::new(0, 1, 0))
            ]

        )
        .with_weight(PROPS_WEIGHT);

    terrain_model_builder.create_model(
        stump_prop.clone(),
        vec![SpawnableAsset::new(SpriteType::TreeStump1)]
    );

    terrain_model_builder.create_model(
        stump_prop.clone(),
        vec![SpawnableAsset::new(SpriteType::TreeStump2)]
    );

    terrain_model_builder.create_model(
        stump_prop.clone(),
        vec![SpawnableAsset::new(SpriteType::TreeStump3)]
    );

    terrain_model_builder.create_model(rock_prop.clone(), vec![SpawnableAsset::new(SpriteType::Rock1)]);
    terrain_model_builder.create_model(rock_prop.clone(), vec![SpawnableAsset::new(SpriteType::Rock2)]);
    terrain_model_builder.create_model(rock_prop.clone(), vec![SpawnableAsset::new(SpriteType::Rock3)]);
    terrain_model_builder.create_model(rock_prop.clone(), vec![SpawnableAsset::new(SpriteType::Rock4)]);

    terrain_model_builder.create_model(plant_prop.clone(), vec![SpawnableAsset::new(SpriteType::Plant1)]);
    terrain_model_builder.create_model(plant_prop.clone(), vec![SpawnableAsset::new(SpriteType::Plant2)]);
    terrain_model_builder.create_model(plant_prop.clone(), vec![SpawnableAsset::new(SpriteType::Plant3)]);
    terrain_model_builder.create_model(plant_prop.clone(), vec![SpawnableAsset::new(SpriteType::Plant4)]);

    socket_collection.add_connections(vec![
        (
            terrain_sockets.props.big_tree_1_base,
            vec![terrain_sockets.props.big_tree_1_base]
        ),
        (
            terrain_sockets.props.big_tree_2_base,
            vec![terrain_sockets.props.big_tree_2_base]
        )
    ]);

    socket_collection
        .add_rotated_connection(
            terrain_sockets.water.layer_up,
            vec![terrain_sockets.props.layer_down]
        )
        .add_rotated_connection(
            terrain_sockets.props.props_down,
            vec![terrain_sockets.water.ground_up]
        );
     
}

pub fn build_world() -> (
    Vec<Vec<SpawnableAsset>>,
    ModelCollection<Cartesian3D>,
    SocketCollection
) {
    let mut socket_collection = SocketCollection::new();
    let terrain_sockets = create_sockets(&mut socket_collection);

    let mut terrain_model_builder = TerrainModelBuilder::new();

    build_dirt_layer(
        &mut terrain_model_builder,
        &terrain_sockets,
        &mut socket_collection
    );

    build_grass_layer(
        &mut terrain_model_builder,
        &terrain_sockets,
        &mut socket_collection
    );

    build_yellow_grass_layer(
        &mut terrain_model_builder,
        &terrain_sockets,
        &mut socket_collection
    );

    build_water_layer(
        &mut terrain_model_builder,
        &terrain_sockets,
        &mut socket_collection
    );

    build_props_layer(
        &mut terrain_model_builder,
        &terrain_sockets,
        &mut socket_collection
    );

    let (assets, models) = terrain_model_builder.into_parts();
    (assets, models, socket_collection)
}
