use std::{collections::HashMap, env, fs, ops::Deref, path::Path};
use bevy::prelude::*;
use serde::Deserialize;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        let constants = match load_constants("src/assets/constants.json") {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to load constants.json: {}", e);
                Constants(HashMap::new())
            }
        };

        app.insert_resource(constants);
    }
}

fn load_constants<P: AsRef<Path>>(path: P) -> Result<Constants, String> {
    let data = fs::read(path.as_ref()).map_err(|e| format!("File read error: {}", e))?;
    serde_json::from_slice::<Constants>(&data).map_err(|e| format!("JSON parse error: {}", e))
}

#[derive(Deserialize, Resource)]
pub struct Constants(pub HashMap<String, i32>);

impl Deref for Constants {
    type Target = HashMap<String, i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
