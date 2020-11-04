// allow the use of macros in schema and models modules
#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::ConnectionResult;
use std::env;
use std::error::Error;

pub mod models;
mod schema;

// establish connection to database
pub fn con_db() -> ConnectionResult<PgConnection> {
    let url = match get_db_url() {
	Ok(v) => v,
	Err(e) => {
	    println!("{}", e);
	    panic!("Error accessing the database URL");
	}
    };

    PgConnection::establish(&url)
}

// insert a post into the database table
pub fn insert_post(db: &PgConnection, post: models::NewPost) {
    use schema::posts;

    diesel::insert_into(posts::table)
	.values(&post)
	.get_result::<models::Post>(db)
	.expect("Error inserting post");
}

// get the database URL
pub fn get_db_url() -> Result<String, Box<dyn Error>> {
    let key = "DATABASE_URL";
    let val = env::var(key)?;

    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_gets_db_url() {
	match get_db_url() {
	    Ok(v) => println!("value is {}", v),
	    Err(e) => {
		println!("{}", e);
		panic!("Error accessing the database URL");
	    }
	}
    }

    #[test]
    fn it_connects_db() {
	match con_db() {
	    Ok(_) => println!("Connect to db successfully"),
	    Err(e) => panic!("Error connecting to db, {:?}", e),
	}
    }
}
