use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, middleware::Logger};
use tera::Tera;
pub mod handlers; // new line

#[macro_use]
extern crate lazy_static;

lazy_static! {
	pub static ref TEMPLATES: Tera = {
	    let mut tera = match Tera::new("templates/**/*.html") {
			Ok(t) => t,
			Err(e) => {
				println!("Parsing error(s): {}", e);
				::std::process::exit(1);
			}
		};
		tera.autoescape_on(vec![".html", ".sql"]);
		tera
	};
}

pub fn start_blog(listener: TcpListener) -> Result<Server, std::io::Error> {
	let srv = HttpServer::new(move || {
		App::new()
		   .app_data(web::Data::new(TEMPLATES.clone()))
		   .wrap(Logger::default()) // enable logger
		   .route("/health", web::get().to(HttpResponse::Ok))
			.service(handlers::index) // new line
	})
	.listen(listener)?
	.run();
	
	Ok(srv)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
	let listener = TcpListener::bind("127.0.0.1:8000")?;
	let server = start_blog(listener)?;
	server.await
}