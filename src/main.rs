use std::{net::TcpListener, thread, fs::File, io::{Write, BufWriter}};

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

const FILE: &str = "./recording.wav";

fn main() {
    let addr = "127.0.0.1:9002";
    let server = TcpListener::bind(addr).unwrap();

    for stream in server.incoming() {
        thread::spawn(move || {
            let callback = |req: &Request, mut response: Response| {
                println!("Received a new ws handshake");
                println!("The request's path is: {}", req.uri().path());
                println!("The request's headers are:");
                for (ref header, _value) in req.headers() {
                    println!("* {}", header);
                }

                // Let's add an additional header to our response to the client.
                let headers = response.headers_mut();
                headers.append("Content-Type", "audio/mpeg".parse().unwrap());

                Ok(response)
            };

            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();
            let mut buffer = BufWriter::new(File::create(FILE).unwrap());

            loop {
                let msg = websocket.read().unwrap();
                if msg.is_binary() || msg.is_text() {
                    // Storing the data to local .wav file 
                    let l = buffer.write(&msg.clone().into_data()).unwrap();
                    println!("buffer len: {l}");

                    buffer.flush().unwrap();
                }
            }
        });
    }
}
