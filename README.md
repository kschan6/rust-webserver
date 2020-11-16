# About
A simple web server built using Rust. It is capable of serving static files and a dynamic page.

## Goals
1. To learn how to build a web server using Rust
2. To learn how to serve static files (e.g. HTML, CSS) using the web server
3. To learn how to interact with a database (PostgreSQL) from the web server
4. To learn how to serve a dynamic page by using the web server

## Prerequisites
In order to set up our program to access the database, please perform the following steps:<br>
1. Install [Diesel CLI](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) with PostgreSQL as the only database dependency:<br>
`cargo install diesel_cli --no-default-features --features postgres`

2. Next, we need to tell Diesel where to find our database. We do this by setting the PostgreSQL [connection URI](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING) to an environment variable called **DATABASE_URL**. For example, we can add the following line to our .bashrc file:<br>
`export DATABASE_URL=postgres://user@localhost:5432/temp_db`<br>
Note: The connection URI on your system may differ depending on the chosen database name, port, etc.

3. Create the PostgreSQL database and run the migration:<br>
`diesel setup`

4. You should see the following output on your terminal (the database name may differ depending on what you have chosen in step 2 above):<br>
> Creating database: temp_db
> Running migration 2020-11-01-031740_create_posts

5. Alright! We should now have a working database and a table called "posts" that allow the web server to store and retrieve posts

## Run the Tests
1. Run the unit tests to make sure the program can connect to the database:<br>
`cargo test`

2. If all the tests pass, you are good to go! Head over to the following section on how to run the web server. In case of failure, please check that you have followed the steps listed in **Prerequisites** section correctly

## How to Use
1. Clone this repository into directory `MYDIR` (you can specify other directory):<br>
`git clone "git@github.com:kschan6/rust-webserver.git" MYDIR`

2. Change into `MYDIR` directory<br>
`cd MYDIR`

3. Fetch all the dependencies and compile the project<br>
`cargo build`

4. When the compilation is done, run the executable<br>
`cargo run`

5. Verify that you see the following output on your terminal
> Hello, rust web server!

6. Open a web browser and go to `localhost:3000`. Voila! You should see the home page served by the Rust web server running on your local machine

7. To test the database access and dynamic page generation features, head over to `http://localhost:3000/minitwitter.html` (or by clicking on the **Minitwitter** hyperlink on the page)

8. Type something in the textarea box and then click the "Post" button. The page should reload after a while and you should see the post listed under the "Past Posts" section. That's it! Our web server is able to serve a page dynamically based on the post content, and it could store/retrieve these posts by interacting with a PostgreSQL database

## References
1. [actix](https://github.com/actix/examples/tree/master/static_index)
2. [diesel](https://diesel.rs/guides/getting-started/)
