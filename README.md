# About
A simple web server built using Rust. It is capable of serving static files and a dynamic page.

## Goals
1. To learn how to build a web server using Rust
2. To learn how to serve static files (e.g. HTML, CSS) using the web server
3. To learn how to interact with a database (PostgreSQL) from the web server
4. To learn how to serve a dynamic page by using the web server

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

## References
1. [actix](https://github.com/actix/examples/tree/master/static_index)
2. [diesel](https://diesel.rs/guides/getting-started/)
