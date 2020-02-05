use serde::{Serialize, Deserialize};
use tiny_http::{Server, Response};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind", content = "value")]
enum Instruction {
    Toggle(usize),
    Oscillate(usize, usize)
}

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();

    for mut request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
                 request.method(),
                 request.url(),
                 request.headers()
        );

        let mut content = String::new();
        request.as_reader().read_to_string(&mut content).unwrap();
        let json: Instruction = serde_json::from_str(&content).unwrap();
        println!("{:?}", json);

        let response = Response::from_string("hello world");
        request.respond(response);
    }
}


