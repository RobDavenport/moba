use std::sync::mpsc::*;

pub struct MessageListener<T> {
    receiver: Receiver<T>,
}

impl<T> MessageListener<T> {
    pub fn new(receiver: Receiver<T>) -> Self {
        Self { receiver }
    }

    pub async fn get_next_message(&self) -> Result<T, RecvError> {
        self.receiver.recv()
    }

    pub fn check_messages(&self) -> Option<Vec<T>> {
        let mut out: Vec<T> = Vec::new();

        loop {
            match self.receiver.try_recv() {
                Ok(msg) => {
                    out.push(msg);
                }
                Err(e) => match e {
                    TryRecvError::Empty => break,
                    TryRecvError::Disconnected => panic!("Message Listener broke!"),
                },
            }
        }

        if out.len() > 0 {
            return Some(out);
        } else {
            return None;
        }
    }
}
