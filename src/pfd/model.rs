use bevy::prelude::*;


#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct Heading(pub f32);

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct Roll(pub f32);

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct Pitch(pub f32);

