use actix_files::Files;
use actix_web::{App, HttpServer, HttpResponse,
		Responder, web};
use serde::{Serialize, Deserialize};
use minitwitter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, rust web server!");

    HttpServer::new(|| {
	App::new()
	    .service(web::resource("/minitwitter")
		     .route(web::post()
			    .to(minitwitter_post)
		     )
	    )
	    .service(Files::new("/", "./static/")
		     .index_file("index.html"))
    })
	.bind("127.0.0.1:3000")?
	.run()
	.await
}

#[derive(Serialize, Deserialize, Debug)]
struct TextObj {
    val: String, // must match the JSON object constructed at the front-end
}

// handle POST request sent to /minitwitter
// TextObj will be deserialized from the request body (a JSON string)
async fn minitwitter_post(obj: web::Json<TextObj>) -> impl Responder {
    println!("obj: {:?}", &obj);
    println!("ori obj: {:?}", obj.0);
    println!("val: {}", obj.0.val);
    println!("val2: {}", obj.val); // not sure why we could access obj.val

    match minitwitter::con_db() {
	Ok(_) => println!("Success connecting to database"),
	Err(e) => {
	    println!("Error connecting to database");
	    println!("{}", e);
	    return HttpResponse::Ok().finish();
	}
    }

    // Serialize the received struct object back into a JSON string
    HttpResponse::Ok().json(obj.0)
}
