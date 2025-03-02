use bevy::prelude::*;
use std::collections::HashMap;

use crate::components::{GridPos, Land, LowerAir, UpperAir};

pub struct TilesPlugin;
impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

#[derive(Resource)]
struct TileMap {
    height: usize,
    width: usize,
    map: HashMap<(i16, i16), Entity>,
    border: HashMap<(i16, i16), Entity>
}

#[derive(Component)]
#[require(GridPos, Land, UpperAir, LowerAir, Sprite)]
pub struct Tile {
    packets: Vec<Entity>,
    radiation: i16,
    change: i16
}

#[derive(Component)]
#[require(GridPos, LowerAir, Sprite)]
pub struct BorderTile {
    opposite: Entity
}