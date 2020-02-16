//use std::sync::mpsc::*;
use tokio::sync::mpsc::{ Receiver, error::TryRecvError };

pub struct MessageListener<T> {
    receiver: Receiver<T>,
}

impl<T> MessageListener<T> {
    pub fn new(receiver: Receiver<T>) -> Self {
        Self { receiver }
    }

    pub async fn get_next_message(&mut self) -> Option<T> {
        self.receiver.recv().await
    }
 
    pub fn check_messages(&mut self) -> Option<Vec<T>> {
        let mut out: Vec<T> = Vec::new();

        loop {
            match self.receiver.try_recv() {
                Ok(msg) => {
                    out.push(msg);
                }
                Err(e) => match e {
                    TryRecvError::Empty => break,
                    TryRecvError::Closed => break, //TODO: is this correct?
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
