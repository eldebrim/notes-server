#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate futures;
extern crate serde;
extern crate tokio;

mod classes;
mod schema;

use self::classes::*;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

use std::env;
use std::io::BufReader;
use std::net::SocketAddr;
use std::str;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

const USAGE: &str = "
    store [title] [note]    Add a new note to the database
    list                    List all stored notes
    show [id]               Show note with the specified ID
    delete [id]             Delete note with specified ID\n";

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    let socket = TcpListener::bind(&addr).unwrap();
    println!("Listening on: {}", addr);
    let done = socket
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        // Logic for each accepted connection
        .for_each(move |socket| {
            let (reader, mut writer) = socket.split();
            // Create a stream of lines for the reader part of the socket
            let reader = io::lines(BufReader::new(reader));
            let process = reader
                // Logic for each line in a stream
                .for_each(move |line| {
                    let connection = establish_connection();
                    let args: Vec<&str> = line.split(' ').collect();
                    // TODO: allow storing multiple notes using one `store` command
                    if args[0] == "store" {
                        Class::create_class(&connection, args[1].to_string(), &args[2].to_string());
                        println!("{:?}", &Class::list(&connection)[0].title);
                        writer.write(Class::display_string(Class::list(&connection)).as_bytes());
                        Ok(())
                    } else if args[0] == "list" {
                        writer.write(Class::display_string(Class::list(&connection)).as_bytes());
                        Ok(())
                    } else if args[0] == "show" {
                        writer.write(
                            Class::display_string(Class::show(
                                args[1].parse::<i32>().unwrap(),
                                &connection,
                            )).as_bytes(),
                        );
                        Ok(())
                    } else if args[0] == "delete" {
                        Class::delete(args[1].parse::<i32>().unwrap(), &connection);
                        writer.write(Class::display_string(Class::list(&connection)).as_bytes());
                        Ok(())
                    } else {
                        writer.write(USAGE.as_bytes());
                        Ok(())
                    }
                }).map_err(|err| println!("Could not process input: {:?}", err));
            tokio::spawn(process);
            Ok(())
        });
    tokio::run(done);
}
