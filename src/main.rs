use actix_files::Files;
use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, rust web server!");

    HttpServer::new(|| {
	App::new()
	    .configure(app_config)
    })
	.bind("127.0.0.1:3000")?
	.run()
	.await
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(Files::new("/", "./static/")
		   .index_file("index.html")
    );
}
