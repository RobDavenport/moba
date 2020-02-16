use std::sync::{Arc, Mutex};

use webrtc_unreliable::Server as RtcServer;

use futures::{
    future::{Fuse, FusedFuture, FutureExt},
    pin_mut, select,
    stream::{FusedStream, FuturesUnordered, Stream, StreamExt},
};

use tokio::sync::mpsc::{Receiver};

use crate::engine::messaging::message_listener::MessageListener;
use crate::engine::messaging::messages::{ OutMessage };

pub struct RtcServerRunner {
    //server_ptr: Arc<Mutex<RtcServer>>,
//rtc_server: RtcServer,
}

impl RtcServerRunner {
    // pub fn new(rtc_server: RtcServer) -> Self {
    //     Self {
    //         server_ptr: Arc::new(Mutex::new(rtc_server)),
    //     }
    //     // Self {
    //     //     rtc_server
    //     // }
    // }

    //TODO: Add way to read "out messages" from game_out_unreliable...
    pub async fn run_rtc_server(mut rtc_server: RtcServer, mut out: Receiver<OutMessage>) -> tokio::task::JoinHandle<()> {
        println!("trying to run rtc_server..");

        tokio::spawn(async move {
            let mut msg_buf = vec![0; 0x10000];
            //let rtc_msg_fut = rtc_server.recv(&mut msg_buf).fuse();
            //let out_msg_fut = out.recv().fuse();
            //pin_mut!(rtc_msg_fut, out_msg_fut);
            println!("STARTED WEBRTC SERVER LOOP!");
            loop {            
                let msg = out.recv().await;
                print!("m");    
                // select! {
                //     // rtc_msg = rtc_msg_fut => {
                //     //     match rtc_msg {
                //     //         Ok(msg) => {print!("got rtc message"),
                //     //         Err(e) => println!("error: {}", e),
                //     //     }
                //     // }
                //     out_msg = out.recv().fuse() => {
                //         match out_msg {
                //             _ => {
                //                 print!("got out message");
                //                 //out_msg_fut.set(out.recv().fuse());
                //             }
                //         }
                //     },
                // };
            }
        })
    }

    // pub async fn send(&mut self, msg: String) {
    //     self.rtc_server.send().await
    // }
}
