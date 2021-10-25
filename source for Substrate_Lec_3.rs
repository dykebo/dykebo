/***************************************************************************** 
Filename: main.rs
Date: 10/25/2021
Author: Fang Lei
Version: 0.1
Description: toy program, 
             echoing data messages between rust server and telnet client
Function list: 1. handle_connection()
******************************************************************************/

use std::io::prelude::*; //io services that let us read from and write to the stream
use std::net::TcpListener; //listen for TCP connections
use std::net::TcpStream; // keeps track of what data it returns to us internally

fn main() {//this is the main function, the entry point to the rust program
    
    //binding to a port 127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to connect rust server");
    
    // Overall, this for loop will process each connection in turn and produce a series of streams for us to handle
    for stream in listener.incoming() {//the incoming method returns an iterator that gives us a sequence of streams 
        let stream = stream.unwrap(); //calling unwrap to terminate program if the stream has any errors
        handle_connection(stream); // implement the functionality to process the specific stream
    }
}

fn handle_connection(mut stream: TcpStream) {//use mut keyword coz its internal state may change
    let mut buffer = [0; 1024]; //dynamically allocate the memory space to hold data
    stream.read(&mut buffer).unwrap(); //save the stream data to buffer

    let contents="Hi client, from Rust Server\r\n\r\n"; //the info displayed on the client node
    let response = format!( // the data block in respond to a client request
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", //200 is the standard success response
        contents.len(), //the length of the contents
        contents //message shown to the client
    );

    stream.write(response.as_bytes()).unwrap(); //calling as_bytes to convert the string data to bytes, before the write operation
    stream.flush().unwrap(); // flush method will wait until all the bytes are written to the connection
    match std::str::from_utf8( &buffer ){ //error handling part
        //successfully recieve message from the client
        Ok(_s) => println!("Message Recieved: {}", String::from_utf8_lossy(&buffer[..])),
        //otherwise, error message is shown to the server
        Err(_e) => panic!("buffer error"),
    }
}