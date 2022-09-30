use simple_websockets::{Event, Responder};
use std::collections::HashMap;

fn main() {
    let event_hub = simple_websockets::launch(8023)
        .expect("failed to listen on port 8023");
    let mut clients: HashMap<u64, Responder> = HashMap::new();

    loop {
        match event_hub.poll_event() {
            Event::Connect(client_id, responder) => {
                clients.insert(client_id, responder);
            },
            Event::Disconnect(client_id) => {
                clients.remove(&client_id);
            },
            Event::Message(client_id, message) => {
                let responder = clients.get(&client_id).unwrap();
                responder.send(message);
            },
        }
    }
}
