use bevy::math::{URect, UVec2};

pub struct TilemapSprite {
    pub name: SpriteType,
    pub pixel_x: u32,
    pub pixel_y: u32
}

pub struct TilemapDefinition {
    pub tile_width: u32,
    pub tile_height: u32,
    pub atlas_width: u32,
    pub atlas_height: u32,
    pub sprites: &'static [TilemapSprite]
}

impl TilemapDefinition {
    pub const fn tile_size(&self) -> UVec2 {
        UVec2::new(self.tile_width, self.tile_height)
    }

    pub const fn atlas_size(&self) -> UVec2 {
        UVec2::new(self.atlas_width, self.atlas_height)

    }

    pub fn sprite_index(&self, name: SpriteType) -> Option<usize> {
        self.sprites
            .iter()
            .position(|sprite| sprite.name == name)
    }

    pub fn sprite_rect(&self, index: usize) -> URect {
        let sprite = &self.sprites[index];
        let min = UVec2::new(sprite.pixel_x, sprite.pixel_y);
        URect::from_corners(min, min + self.tile_size())
    }
}


#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum GreenGrass {
    GreenGrass,
    CornerInTopLeft,
    CornerInTopRight,
    CornerInBottomLeft,
    CornerInBottomRight,
    CornerOutTopLeft,
    CornerOutTopRight,
    CornerOutBottomLeft,
    CornerOutBottomRight,
    SideTop,
    SideRight,
    SideLeft,
    SideBottom
}


#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum YellowGrass {
    YellowGrass,
    CornerInTopLeft,
    CornerInTopRight,
    CornerInBottomLeft,
    CornerInBottomRight,
    CornerOutTopLeft,
    CornerOutTopRight,
    CornerOutBottomLeft,
    CornerOutBottomRight,
    SideTop,
    SideRight,
    SideLeft,
    SideBottom
}


#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Water {
    Water,
    CornerInTopLeft,
    CornerInTopRight,
    CornerInBottomLeft,
    CornerInBottomRight,
    CornerOutTopLeft,
    CornerOutTopRight,
    CornerOutBottomLeft,
    CornerOutBottomRight,
    SideTop,
    SideRight,
    SideLeft,
    SideBottom
}


#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum BigTreeOne {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}


#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum BigTreeTwo {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}


#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Plant {
    One,
    Two,
    Three,
    Four
}


#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Rock {
    One,
    Two,
    Three,
    Four
}


#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum SmallTree {
    Top,
    Bottom,
}


#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TreeStump {
    One,
    Two,
    Three
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum SpriteType {
    Dirt,
    GreenGrass(GreenGrass),
    YellowGrass(YellowGrass),
    Water(Water),
    BigTreeOne(BigTreeOne),
    BigTreeTwo(BigTreeTwo),
    Plant(Plant),
    Rock(Rock),
    SmallTree(SmallTree),
    TreeStump(TreeStump)
}

pub const TILEMAP: TilemapDefinition = TilemapDefinition {
    tile_width: 32,
    tile_height: 32,
    atlas_width: 256,
    atlas_height: 320,
    sprites: &[
        TilemapSprite {
            name: SpriteType::Dirt,
            pixel_x: 128,
            pixel_y: 0
        },
        TilemapSprite {
            name: SpriteType::GreenGrass(GreenGrass::GreenGrass),
            pixel_x: 160,
            pixel_y: 0,
        },
        TilemapSprite {
            name: SpriteType::GreenGrass(GreenGrass::CornerInTopLeft),
            pixel_x: 192,
            pixel_y: 0,
        },
        TilemapSprite {
            name: SpriteType::GreenGrass(GreenGrass::CornerInTopRight),
            pixel_x: 224,
            pixel_y: 0,
        },
        TilemapSprite {
            name: SpriteType::GreenGrass(GreenGrass::CornerInBottomLeft),
            pixel_x: 192,
            pixel_y: 32,
        },
        TilemapSprite {
            name: SpriteType::GreenGrass(GreenGrass::CornerInBottomRight),
            pixel_x: 224,
            pixel_y: 32,
        },
        TilemapSprite {
            name: SpriteType::GreenGrass(GreenGrass::CornerOutTopLeft),
            pixel_x: 0,
            pixel_y: 64,
        },
        TilemapSprite {
            name: SpriteType::GreenGrass(GreenGrass::CornerOutTopRight),
            pixel_x: 32,
            pixel_y: 64,
        },
        TilemapSprite {
            name: SpriteType::GreenGrass(GreenGrass::CornerOutBottomLeft),
            pixel_x: 0,
            pixel_y: 96,
        },
        TilemapSprite {
            name: SpriteType::GreenGrass(GreenGrass::CornerOutBottomRight),
            pixel_x: 32,
            pixel_y: 96,
        },
        TilemapSprite {
            name: SpriteType::GreenGrass(GreenGrass::SideTop),
            pixel_x: 64,
            pixel_y: 64,
        },
        TilemapSprite {
            name: SpriteType::GreenGrass(GreenGrass::SideRight),
            pixel_x: 96,
            pixel_y: 64,
        },
        TilemapSprite {
            name: SpriteType::GreenGrass(GreenGrass::SideLeft),
            pixel_x: 64,
            pixel_y: 96,
        },
        TilemapSprite {
            name: SpriteType::GreenGrass(GreenGrass::SideBottom),
            pixel_x: 96,
            pixel_y: 96,
        },
        TilemapSprite {
            name: SpriteType::YellowGrass(YellowGrass::YellowGrass),
            pixel_x: 0,
            pixel_y: 256,
        },
        TilemapSprite {
            name: SpriteType::YellowGrass(YellowGrass::CornerInTopLeft),
            pixel_x: 32,
            pixel_y: 256,
        },
        TilemapSprite {
            name: SpriteType::YellowGrass(YellowGrass::CornerInTopRight),
            pixel_x: 64,
            pixel_y: 256,
        },
        TilemapSprite {
            name: SpriteType::YellowGrass(YellowGrass::CornerInBottomLeft),
            pixel_x: 32,
            pixel_y: 288,
        },
        TilemapSprite {
            name: SpriteType::YellowGrass(YellowGrass::CornerInBottomRight),
            pixel_x: 64,
            pixel_y: 288,
        },
        TilemapSprite {
            name: SpriteType::YellowGrass(YellowGrass::CornerOutTopLeft),
            pixel_x: 96,
            pixel_y: 256,
        },
        TilemapSprite {
            name: SpriteType::YellowGrass(YellowGrass::CornerOutTopRight),
            pixel_x: 128,
            pixel_y: 256,
        },
        TilemapSprite {
            name: SpriteType::YellowGrass(YellowGrass::CornerOutBottomLeft),
            pixel_x: 96,
            pixel_y: 288,
        },
        TilemapSprite {
            name: SpriteType::YellowGrass(YellowGrass::CornerOutBottomRight),
            pixel_x: 128,
            pixel_y: 288,
        },
        TilemapSprite {
            name: SpriteType::YellowGrass(YellowGrass::SideTop),
            pixel_x: 160,
            pixel_y: 256,
        },
        TilemapSprite {
            name: SpriteType::YellowGrass(YellowGrass::SideRight),
            pixel_x: 192,
            pixel_y: 256,
        },
        TilemapSprite {
            name: SpriteType::YellowGrass(YellowGrass::SideLeft),
            pixel_x: 160,
            pixel_y: 288,
        },
        TilemapSprite {
            name: SpriteType::YellowGrass(YellowGrass::SideBottom),
            pixel_x: 192,
            pixel_y: 288,
        },
        TilemapSprite {
            name: SpriteType::Water(Water::Water),
            pixel_x: 32,
            pixel_y: 192,
        },
        TilemapSprite {
            name: SpriteType::Water(Water::CornerInTopLeft),
            pixel_x: 64,
            pixel_y: 192,
        },
        TilemapSprite {
            name: SpriteType::Water(Water::CornerInTopRight),
            pixel_x: 96,
            pixel_y: 192,
        },
        TilemapSprite {
            name: SpriteType::Water(Water::CornerInBottomLeft),
            pixel_x: 64,
            pixel_y: 224,
        },
        TilemapSprite {
            name: SpriteType::Water(Water::CornerInBottomRight),
            pixel_x: 96,
            pixel_y: 224,
        },
        TilemapSprite {
            name: SpriteType::Water(Water::CornerOutTopLeft),
            pixel_x: 128,
            pixel_y: 192,
        },
        TilemapSprite {
            name: SpriteType::Water(Water::CornerOutTopRight),
            pixel_x: 160,
            pixel_y: 192,
        },
        TilemapSprite {
            name: SpriteType::Water(Water::CornerOutBottomLeft),
            pixel_x: 128,
            pixel_y: 224,
        },
        TilemapSprite {
            name: SpriteType::Water(Water::CornerOutBottomRight),
            pixel_x: 160,
            pixel_y: 224,
        },
        TilemapSprite {
            name: SpriteType::Water(Water::SideTop),
            pixel_x: 192,
            pixel_y: 192,
        },
        TilemapSprite {
            name: SpriteType::Water(Water::SideRight),
            pixel_x: 224,
            pixel_y: 192,
        },
        TilemapSprite {
            name: SpriteType::Water(Water::SideLeft),
            pixel_x: 192,
            pixel_y: 224,
        },
        TilemapSprite {
            name: SpriteType::Water(Water::SideBottom),
            pixel_x: 224,
            pixel_y: 224,
        },
        TilemapSprite {
            name: SpriteType::BigTreeOne(BigTreeOne::TopLeft),
            pixel_x: 0,
            pixel_y: 0,
        },
        TilemapSprite {
            name: SpriteType::BigTreeOne(BigTreeOne::TopRight),
            pixel_x: 32,
            pixel_y: 0,
        },
        TilemapSprite {
            name: SpriteType::BigTreeOne(BigTreeOne::BottomLeft),
            pixel_x: 0,
            pixel_y: 32,
        },
        TilemapSprite {
            name: SpriteType::BigTreeOne(BigTreeOne::BottomRight),
            pixel_x: 32,
            pixel_y: 32,
        },
        TilemapSprite {
            name: SpriteType::BigTreeTwo(BigTreeTwo::TopLeft),
            pixel_x: 64,
            pixel_y: 0,
        },
        TilemapSprite {
            name: SpriteType::BigTreeTwo(BigTreeTwo::TopRight),
            pixel_x: 96,
            pixel_y: 0,
        },
        TilemapSprite {
            name: SpriteType::BigTreeTwo(BigTreeTwo::BottomLeft),
            pixel_x: 64,
            pixel_y: 32,
        },
        TilemapSprite {
            name: SpriteType::BigTreeTwo(BigTreeTwo::BottomRight),
            pixel_x: 96,
            pixel_y: 32,
        },
        TilemapSprite {
            name: SpriteType::Plant(Plant::One),
            pixel_x: 128,
            pixel_y: 64,
        },
        TilemapSprite {
            name: SpriteType::Plant(Plant::Two),
            pixel_x: 160,
            pixel_y: 64,
        },
        TilemapSprite {
            name: SpriteType::Plant(Plant::Three),
            pixel_x: 192,
            pixel_y: 64,
        },
        TilemapSprite {
            name: SpriteType::Plant(Plant::Four),
            pixel_x: 224,
            pixel_y: 64,
        },
        TilemapSprite {
            name: SpriteType::Rock(Rock::One),
            pixel_x: 0,
            pixel_y: 128,
        },
        TilemapSprite {
            name: SpriteType::Rock(Rock::Two),
            pixel_x: 32,
            pixel_y: 128,
        },
        TilemapSprite {
            name: SpriteType::Rock(Rock::Three),
            pixel_x: 64,
            pixel_y: 128,
        },
        TilemapSprite {
            name: SpriteType::Rock(Rock::Four),
            pixel_x: 96,
            pixel_y: 128,
        },
        TilemapSprite {
            name: SpriteType::SmallTree(SmallTree::Top),
            pixel_x: 128,
            pixel_y: 128,
        },
        TilemapSprite {
            name: SpriteType::SmallTree(SmallTree::Bottom),
            pixel_x: 128,
            pixel_y: 160,
        },
        TilemapSprite {
            name: SpriteType::TreeStump(TreeStump::One),
            pixel_x: 192,
            pixel_y: 128,
        },
        TilemapSprite {
            name: SpriteType::TreeStump(TreeStump::Two),
            pixel_x: 224,
            pixel_y: 128,
        },
        TilemapSprite {
            name: SpriteType::TreeStump(TreeStump::Three),
            pixel_x: 0,
            pixel_y: 192,
        },
    ]
};
