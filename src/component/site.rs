use crate::CONFIG;

use actix_web::{ HttpServer, App, web, };

pub mod api;
pub mod user;

pub async fn start() {
	let _ = actix_start().await;
}

pub async fn actix_start() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
		.service(
			web::scope("/api/v1")
				.service(api::get_info_forklift)
				.service(api::get_info_space)
		)
		.service(
			web::scope("/web")
				.service(user::index)
		)
	})
	.bind(CONFIG.address)?//CONFIG
	.run()
	.await
}
