extern crate ws;
use ws::*;
use ws::util::Token;

struct Server {
    senders: Vec<Sender>
}

impl Server {
    pub fn new() -> Self {
        Self {
            senders: Vec::new()
        }
    }

    pub fn add_user(&mut self, sender: Sender) {
        self.senders.push(sender)
    }

    pub fn broadcast_all(&self, msg: Message) {
        for sender in self.senders.iter() {
            sender.send(msg.clone());
        }
    }

    pub fn remove_user(&mut self, token: Token) {
        self.senders.retain(|out| token != out.token());
    }
}

impl Factory for Server {
    type Handler = Client;

    fn connection_made(&mut self, ws: Sender) -> Client {
        self.add_user(ws.clone());
        Client {
            server: self,
            out: ws
        }
    }
}

struct Client { 
    server: *mut Server,
    out: Sender
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("User connected successfully...");
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Got message: {}", msg);
        unsafe {
            self.server.as_ref().unwrap().broadcast_all(msg);
        }
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, _reason: &str) {
        println!("User left because: {:?}", code);
        unsafe {
            self.server.as_mut().unwrap().remove_user(self.out.token());
        }
    }
}

fn main() {
    let websocket = WebSocket::<Server>::new(Server::new()).unwrap();
    websocket.listen("127.0.0.1:8000");
}
