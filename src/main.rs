use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, rust web server!");

    HttpServer::new(|| {
	App::new().service(Files::new("/", "./static/")
			   .index_file("index.html"))
    })
	.bind("127.0.0.1:3000")?
	.run()
	.await
}
