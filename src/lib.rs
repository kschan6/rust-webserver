//#[macro_use]
//extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::ConnectionResult;
use std::env;
use std::error::Error;

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
