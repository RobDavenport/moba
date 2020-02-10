use std::collections::VecDeque;
use std::sync::mpsc::*;

pub struct MessageListener<T> {
    receiver: Receiver<T>,
}

impl<T> MessageListener<T> {
    pub fn new(receiver: Receiver<T>) -> Self {
        Self { receiver }
    }

    pub fn check_messages(&self) -> Option<VecDeque<T>> {
        let mut out: VecDeque<T> = VecDeque::new();

        loop {
            match self.receiver.try_recv() {
                Ok(msg) => {
                    out.push_back(msg);
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
