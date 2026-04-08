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
pub enum SpriteType {
    Dirt,
    GreenGrass,
    GreenGrassCornerInTopLeft,
    GreenGrassCornerInTopRight,
    GreenGrassCornerInBottomLeft,
    GreenGrassCornerInBottomRight,
    GreenGrassCornerOutTopLeft,
    GreenGrassCornerOutTopRight,
    GreenGrassCornerOutBottomLeft,
    GreenGrassCornerOutBottomRight,
    GreenGrassSideTop,
    GreenGrassSideRight,
    GreenGrassSideLeft,
    GreenGrassSideBottom,
    YellowGrass,
    YellowGrassCornerInTopLeft,
    YellowGrassCornerInTopRight,
    YellowGrassCornerInBottomLeft,
    YellowGrassCornerInBottomRight,
    YellowGrassCornerOutTopLeft,
    YellowGrassCornerOutTopRight,
    YellowGrassCornerOutBottomLeft,
    YellowGrassCornerOutBottomRight,
    YellowGrassSideTop,
    YellowGrassSideRight,
    YellowGrassSideLeft,
    YellowGrassSideBottom,
    Water,
    WaterCornerInTopLeft,
    WaterCornerInTopRight,
    WaterCornerInBottomLeft,
    WaterCornerInBottomRight,
    WaterCornerOutTopLeft,
    WaterCornerOutTopRight,
    WaterCornerOutBottomLeft,
    WaterCornerOutBottomRight,
    WaterSideTop,
    WaterSideRight,
    WaterSideLeft,
    WaterSideBottom,
    BigTree1TopLeft,
    BigTree1TopRight,
    BigTree1BottomLeft,
    BigTree1BottomRight,
    BigTree2TopLeft,
    BigTree2TopRight,
    BigTree2BottomLeft,
    BigTree2BottomRight,
    Plant1,
    Plant2,
    Plant3,
    Plant4,
    Rock1,
    Rock2,
    Rock3,
    Rock4,
    SmallTreeTop,
    SmallTreeBottom,
    TreeStump1,
    TreeStump2,
    TreeStump3
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
            name: SpriteType::GreenGrass,
            pixel_x: 160,
            pixel_y: 0,
        },
        TilemapSprite {
            name: SpriteType::GreenGrassCornerInTopLeft,
            pixel_x: 192,
            pixel_y: 0,
        },
        TilemapSprite {
            name: SpriteType::GreenGrassCornerInTopRight,
            pixel_x: 224,
            pixel_y: 0,
        },
        TilemapSprite {
            name: SpriteType::GreenGrassCornerInBottomLeft,
            pixel_x: 192,
            pixel_y: 32,
        },
        TilemapSprite {
            name: SpriteType::GreenGrassCornerInBottomRight,
            pixel_x: 224,
            pixel_y: 32,
        },
        TilemapSprite {
            name: SpriteType::GreenGrassCornerOutTopLeft,
            pixel_x: 0,
            pixel_y: 64,
        },
        TilemapSprite {
            name: SpriteType::GreenGrassCornerOutTopRight,
            pixel_x: 32,
            pixel_y: 64,
        },
        TilemapSprite {
            name: SpriteType::GreenGrassCornerOutBottomLeft,
            pixel_x: 0,
            pixel_y: 96,
        },
        TilemapSprite {
            name: SpriteType::GreenGrassCornerOutBottomRight,
            pixel_x: 32,
            pixel_y: 96,
        },
        TilemapSprite {
            name: SpriteType::GreenGrassSideTop,
            pixel_x: 64,
            pixel_y: 64,
        },
        TilemapSprite {
            name: SpriteType::GreenGrassSideRight,
            pixel_x: 96,
            pixel_y: 64,
        },
        TilemapSprite {
            name: SpriteType::GreenGrassSideLeft,
            pixel_x: 64,
            pixel_y: 96,
        },
        TilemapSprite {
            name: SpriteType::GreenGrassSideBottom,
            pixel_x: 96,
            pixel_y: 96,
        },
        TilemapSprite {
            name: SpriteType::YellowGrass,
            pixel_x: 0,
            pixel_y: 256,
        },
        TilemapSprite {
            name: SpriteType::YellowGrassCornerInTopLeft,
            pixel_x: 32,
            pixel_y: 256,
        },
        TilemapSprite {
            name: SpriteType::YellowGrassCornerInTopRight,
            pixel_x: 64,
            pixel_y: 256,
        },
        TilemapSprite {
            name: SpriteType::YellowGrassCornerInBottomLeft,
            pixel_x: 32,
            pixel_y: 288,
        },
        TilemapSprite {
            name: SpriteType::YellowGrassCornerInBottomRight,
            pixel_x: 64,
            pixel_y: 288,
        },
        TilemapSprite {
            name: SpriteType::YellowGrassCornerOutTopLeft,
            pixel_x: 96,
            pixel_y: 256,
        },
        TilemapSprite {
            name: SpriteType::YellowGrassCornerOutTopRight,
            pixel_x: 128,
            pixel_y: 256,
        },
        TilemapSprite {
            name: SpriteType::YellowGrassCornerOutBottomLeft,
            pixel_x: 96,
            pixel_y: 288,
        },
        TilemapSprite {
            name: SpriteType::YellowGrassCornerOutBottomRight,
            pixel_x: 128,
            pixel_y: 288,
        },
        TilemapSprite {
            name: SpriteType::YellowGrassSideTop,
            pixel_x: 160,
            pixel_y: 256,
        },
        TilemapSprite {
            name: SpriteType::YellowGrassSideRight,
            pixel_x: 192,
            pixel_y: 256,
        },
        TilemapSprite {
            name: SpriteType::YellowGrassSideLeft,
            pixel_x: 160,
            pixel_y: 288,
        },
        TilemapSprite {
            name: SpriteType::YellowGrassSideBottom,
            pixel_x: 192,
            pixel_y: 288,
        },
        TilemapSprite {
            name: SpriteType::Water,
            pixel_x: 32,
            pixel_y: 192,
        },
        TilemapSprite {
            name: SpriteType::WaterCornerInTopLeft,
            pixel_x: 64,
            pixel_y: 192,
        },
        TilemapSprite {
            name: SpriteType::WaterCornerInTopRight,
            pixel_x: 96,
            pixel_y: 192,
        },
        TilemapSprite {
            name: SpriteType::WaterCornerInBottomLeft,
            pixel_x: 64,
            pixel_y: 224,
        },
        TilemapSprite {
            name: SpriteType::WaterCornerInBottomRight,
            pixel_x: 96,
            pixel_y: 224,
        },
        TilemapSprite {
            name: SpriteType::WaterCornerOutTopLeft,
            pixel_x: 128,
            pixel_y: 192,
        },
        TilemapSprite {
            name: SpriteType::WaterCornerOutTopRight,
            pixel_x: 160,
            pixel_y: 192,
        },
        TilemapSprite {
            name: SpriteType::WaterCornerOutBottomLeft,
            pixel_x: 128,
            pixel_y: 224,
        },
        TilemapSprite {
            name: SpriteType::WaterCornerOutBottomRight,
            pixel_x: 160,
            pixel_y: 224,
        },
        TilemapSprite {
            name: SpriteType::WaterSideTop,
            pixel_x: 192,
            pixel_y: 192,
        },
        TilemapSprite {
            name: SpriteType::WaterSideRight,
            pixel_x: 224,
            pixel_y: 192,
        },
        TilemapSprite {
            name: SpriteType::WaterSideLeft,
            pixel_x: 192,
            pixel_y: 224,
        },
        TilemapSprite {
            name: SpriteType::WaterSideBottom,
            pixel_x: 224,
            pixel_y: 224,
        },
        TilemapSprite {
            name: SpriteType::BigTree1TopLeft,
            pixel_x: 0,
            pixel_y: 0,
        },
        TilemapSprite {
            name: SpriteType::BigTree1TopRight,
            pixel_x: 32,
            pixel_y: 0,
        },
        TilemapSprite {
            name: SpriteType::BigTree1BottomLeft,
            pixel_x: 0,
            pixel_y: 32,
        },
        TilemapSprite {
            name: SpriteType::BigTree1BottomRight,
            pixel_x: 32,
            pixel_y: 32,
        },
        TilemapSprite {
            name: SpriteType::BigTree2TopLeft,
            pixel_x: 64,
            pixel_y: 0,
        },
        TilemapSprite {
            name: SpriteType::BigTree2TopRight,
            pixel_x: 96,
            pixel_y: 0,
        },
        TilemapSprite {
            name: SpriteType::BigTree2BottomLeft,
            pixel_x: 64,
            pixel_y: 32,
        },
        TilemapSprite {
            name: SpriteType::BigTree2BottomRight,
            pixel_x: 96,
            pixel_y: 32,
        },
        TilemapSprite {
            name: SpriteType::Plant1,
            pixel_x: 128,
            pixel_y: 64,
        },
        TilemapSprite {
            name: SpriteType::Plant2,
            pixel_x: 160,
            pixel_y: 64,
        },
        TilemapSprite {
            name: SpriteType::Plant3,
            pixel_x: 192,
            pixel_y: 64,
        },
        TilemapSprite {
            name: SpriteType::Plant4,
            pixel_x: 224,
            pixel_y: 64,
        },
        TilemapSprite {
            name: SpriteType::Rock1,
            pixel_x: 0,
            pixel_y: 128,
        },
        TilemapSprite {
            name: SpriteType::Rock2,
            pixel_x: 32,
            pixel_y: 128,
        },
        TilemapSprite {
            name: SpriteType::Rock3,
            pixel_x: 64,
            pixel_y: 128,
        },
        TilemapSprite {
            name: SpriteType::Rock4,
            pixel_x: 96,
            pixel_y: 128,
        },
        TilemapSprite {
            name: SpriteType::SmallTreeTop,
            pixel_x: 128,
            pixel_y: 128,
        },
        TilemapSprite {
            name: SpriteType::SmallTreeBottom,
            pixel_x: 128,
            pixel_y: 160,
        },
        TilemapSprite {
            name: SpriteType::TreeStump1,
            pixel_x: 192,
            pixel_y: 128,
        },
        TilemapSprite {
            name: SpriteType::TreeStump2,
            pixel_x: 224,
            pixel_y: 128,
        },
        TilemapSprite {
            name: SpriteType::TreeStump3,
            pixel_x: 0,
            pixel_y: 192,
        },
    ]
};
