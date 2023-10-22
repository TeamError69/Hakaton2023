use serde::{Serialize, Deserialize};

use std::borrow::Cow;
use std::collections::HashMap;

use std::fs::File;
use std::io::Read;


#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
	check_point_id: u32,
	check_point_name: Cow<'static, str>,
	next_check_point_distance: u32,
}

impl Object {
	fn new(check_point_id: u32, check_point_name: Cow<'static, str>, next_check_point_distance: u32) -> Object {
		Object {
			check_point_id,
			check_point_name,
			next_check_point_distance,
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapData {
	path_sequence: Vec<Object>,
	target_rack_id: Cow<'static, str>,
}

impl MapData {
	fn new(path_sequence: Vec<Object>, target_rack_id: Cow<'static, str>) -> MapData {
		MapData {
			path_sequence,
			target_rack_id,
		}
	}
}

pub fn read_config_space(path: &str) -> Result<HashMap<u32, MapData>, Box<dyn std::error::Error>> {
	let mut file_data = String::new();
	let mut file = File::open(path)?;
	file.read_to_string(&mut file_data)?;
	Ok(serde_json::from_str::<HashMap<u32, MapData>>(&file_data)?)
} 
