use super::schema::posts;
use chrono::naive::NaiveDateTime;

// 1. used for retrieving post(s) from databse table
// 2. must match the table structure in schema.rs
// 3. also used for satisfying the trait `diesel::Queryable<(diesel::sql_types::Integer, diesel::sql_types::Text, diesel::sql_types::Timestamp), _>` when get_results from RunQueryDsl trait is used
#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub body: String,
    pub published: NaiveDateTime,
}

// struct used for inserting a post into database table
#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub body: &'a str,
}
