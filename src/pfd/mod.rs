use bevy::prelude::*;

use crate::pfd::{
    model::{Heading, Pitch, Roll}, 
    horizon::HorizonPlugin
};

use self::instruments::InstrumentsPlugin;

mod instruments;
mod horizon;
mod model;

pub struct PfdPlugin;

impl Plugin for PfdPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Heading>();
        app.init_resource::<Roll>();
        app.init_resource::<Pitch>();
        app.add_plugins((InstrumentsPlugin, HorizonPlugin));
    }
}