use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::web::Json;

use logic_module::library::work_space::MapData;
use std::collections::HashMap;

#[get("/info_space")]
async fn get_info_space() -> Json::<HashMap<u32, MapData>> {
	web::Json::<HashMap<u32, MapData>>(HashMap::new())
}

#[get("/info_forklift/{warehouse_id}/{id}")]
async fn get_info_forklift(data: web::Path<(u32, u32)>) -> impl Responder {
	let (warehouse_id, id) = data.into_inner();
	format!("{}:{}", warehouse_id, id) //Data base response
}
