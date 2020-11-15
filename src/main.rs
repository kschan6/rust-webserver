use actix_files::Files;
use actix_web::{App, HttpServer, HttpResponse,
		Responder, web};
use serde::{Serialize, Deserialize};
use minitwitter;
use minitwitter::models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, rust web server!");

    HttpServer::new(|| {
	App::new()
	    .service(web::resource("/minitwitter.html")
		     .route(web::post()
			    .to(minitwitter_post)
		     )
		     .route(web::get()
			    .to(minitwitter_get)
		     )
	    )
	    .service(Files::new("/", "./static/")
		     .index_file("index.html")
	    )
    })
	.bind("127.0.0.1:3000")?
	.run()
	.await
}

#[derive(Serialize, Deserialize, Debug)]
struct TextObj {
    val: String, // must match the JSON object constructed at the front-end
}

// handle POST request sent to /minitwitter.html
// TextObj will be deserialized from the request body (a JSON string)
async fn minitwitter_post(obj: web::Json<TextObj>) -> impl Responder {
    println!("obj: {:?}", &obj);
    println!("ori obj: {:?}", obj.0);
    println!("val: {}", obj.0.val);
    println!("val2: {}", obj.val); // not sure why we could access obj.val

    let db = match minitwitter::con_db() {
	Ok(db) => {
	    println!("Success connecting to database");
	    db
	}
	Err(e) => {
	    println!("Error connecting to database");
	    println!("{}", e);
	    return HttpResponse::Ok().finish();
	}
    };

    let post = models::NewPost {
	body: &obj.val,
    };

    minitwitter::insert_post(&db, post);

    // Serialize the received struct object back into a JSON string
    HttpResponse::Ok().json(obj.0)
}

// handle GET request sent to /minitwitter.html
// dynamically generate an HTML page that: i) contains all posts ii) allows user to insert a new post
fn minitwitter_get() -> HttpResponse {
    let db = match minitwitter::con_db() {
	Ok(db) => {
	    println!("Success connecting to database");
	    db
	}
	Err(e) => {
	    println!("Error connecting to database");
	    println!("{}", e);
	    return HttpResponse::Ok().finish();
	}
    };

    let posts = match minitwitter::show_posts(&db) {
	Ok(v) => v,
	Err(e) => {
	    println!("Error showing posts");
	    println!("{}", e);
	    return HttpResponse::Ok().finish();
	}
    };

    let mut out_post = String::new();

    // concatenate all the posts and output DOM elements accordingly
    for post in &posts {
	println!("id: {}", post.id);
	println!("body: {}", post.body);
	println!("published: {}", post.published);

	// the main div tag wrapping the post content
	out_post = format!("{}<div class=\"post-cell\">", out_post);
	
	// wrap <div> tags around the post body
	out_post = format!("{}<div class=\"post-body\">{}", out_post, post.body);
	out_post = format!("{}</div>", out_post);

	// now add the timestamp part
	let pretty_time = post.published.format("%Y-%m-%d %H:%M").to_string();
	
	out_post = format!("{}<div class=\"post-time\">{}", out_post, pretty_time);
	out_post = format!("{}</div>", out_post);

	// the matching </div> tag of the post content
	out_post = format!("{}</div>", out_post);
    }

    // generate string for the complete HTML document
    let out_html = format!("{}{}{}", OUT_HTML1, out_post, OUT_HTML2);

    // serve the dynamically-generated HTML page!
    HttpResponse::Ok()
	.content_type("text/html")
	.body(out_html)
}

static OUT_HTML1: &'static str = "\
<!DOCTYPE html>
<html>
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <link href=\"index.css\" rel=\"stylesheet\">
    <link href=\"minitwitter.css\" rel=\"stylesheet\">
    <title>Minitwitter</title>
</head>
<body>
<header>
    <a href=\"index.html\">Home</a>
    <a href=\"#\">Minitwitter</a>
</header>
<h1>Share Your Thoughts</h1>
    <section id=\"sec-text\" class=\"text-width-med\">
	  <textarea id=\"sec-textarea\"
		    maxlength=\"140\"
		    required
		    placeholder=\"Write something...\"></textarea>
	  <button type=\"button\">Post</button>
    </section>
<h2>Past Posts</h2>
    <section id=\"sec-post\" class=\"text-width-med\">
";

static OUT_HTML2: &'static str = "\
    </section>
    <script src=\"minitwitter.js\"></script>
</body>
</html>
";
