Asynchronous TCP server for storing notes in a PostgreSQL database using tokio and diesel. 

[![Build Status](http://98.232.188.86:30427/job/notes-server/badge/icon)](http://98.232.188.86:30427/job/notes-server/)

## Requirements

* rustc 1.30.0-nightly.
* A [Diesel supported database](http://diesel.rs/guides/getting-started/).

## Installation
To install directly to your .cargo directory:
```sh
cargo +nightly install --git https://github.com/eldebrim/notes-server notes-server
```
Or if you want to build from source:
```sh
git clone https://github.com/eldebrim/notes-server.git && cd notes-server
cargo build
```

## Usage
Specify the socket address as an argument (127.0.0.1:8080 will be used by default):
```sh
notes-server 127.0.0.1:4000
```
Connect to the socket using telnet:
```sh
telnet localhost 4000
```
Now you can use the following commands to interact with the database:
```sh
help
    store [title] [note]    Add a new note to the database
    list                    List all stored notes
    show [id]               Show note with the specified ID
    delete [id]             Delete note with specified ID
```
