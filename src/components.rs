use bevy::prelude::*;

#[derive(Component, Default)]
pub struct GridPos(pub IVec2);





#[derive(Component, Default)]
pub struct Humidity(pub u16);

#[derive(Component, Default)]
pub struct Temperature(pub u16);

#[derive(Component, Default)]
pub struct Pressure(pub u16);





#[derive(Component, Default)]
pub struct Water(pub u16);

#[derive(Component, Default)]
pub enum Vegetation {
    #[default]
    None,
    Grass(i8),
    Forest(i8),
    Swamp(i8),

}





#[derive(Component, Default)]
#[require(Water, Vegetation, Temperature)]
pub struct Land {
    elevation: i16,
    flooded: bool,

}

#[derive(Component, Default)]
#[require(Temperature)]
pub struct Sea;





#[derive(Component, Default)]
pub struct Atmosphere;

#[derive(Component, Default)]
#[require(Temperature, Pressure, Humidity, Atmosphere)]
pub struct LowerAir;

#[derive(Component, Default)]
#[require(Temperature, Pressure, Humidity, Atmosphere)]
pub struct UpperAir;





#[derive(Component, Default)]
#[require(Temperature, Pressure, Humidity, Transform)]
pub struct AirPacket;

#[derive(Component)]
#[require(AirPacket)]
pub struct Cloud;

