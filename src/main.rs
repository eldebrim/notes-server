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

pub fn establish_connection() -> PgConnection {
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
        .for_each(move |socket| {
            let buf = Vec::new();
            let (reader, mut writer) = socket.split();
            let reader = BufReader::new(reader);
            let process = io::read_until(reader, '\n' as u8, buf)
                .map(move |(_, buf)| {
                    let connection = establish_connection();
                    let args: Vec<&str> = str::from_utf8(&buf).unwrap().split(' ').collect();
                    if args[0] == "store" {
                        Class::create_class(&connection, args[1].to_string(), &args[2].to_string());
                        println!("{:?}", &Class::list(&connection)[0].title);
                        writer.write(Class::display_string(Class::list(&connection)).as_bytes());
                    } else if args[0] == "list\r\n" {
                        println!("incoming: {:?}", str::from_utf8(&buf).unwrap());
                        writer.write(Class::display_string(Class::list(&connection)).as_bytes());
                    } else if args[0] == "show\r\n" {
                        writer.write(
                            Class::display_string(Class::show(
                                args[1].parse::<i32>().unwrap(),
                                &connection,
                            )).as_bytes(),
                        );
                    }
                }).then(|_| Ok(()));
            tokio::spawn(process);
            Ok(())
        });
    tokio::run(done);
}
