use webrtc_unreliable::Server as RtcServer;

use tokio::sync::mpsc::Receiver;

use crate::engine::messaging::messages::OutMessage;

use futures::{
    future::FutureExt,
    select,
    stream::StreamExt,
};


pub struct RtcServerRunner { }

impl RtcServerRunner {
    pub fn run_rtc_server(
        mut rtc_server: RtcServer,
        mut out: Receiver<OutMessage>,
    ) -> tokio::task::JoinHandle<()> {
        println!("trying to run rtc_server..");

        tokio::spawn(async move {
            let mut msg_buf = vec![0; 0x10000];
            println!("STARTED WEBRTC SERVER LOOP!");
            loop {
                select! {
                    rtc_msg = rtc_server.recv(&mut msg_buf).fuse() => {
                        match rtc_msg {
                            Ok(msg) => println!("got rtc message"),
                            Err(e) => println!("error: {}", e),
                        }
                    },
                    out_msg = out.recv().fuse() => {
                        match out_msg {
                            _ => {
                                ()
                                //out_msg_fut.set(out.recv().fuse());
                            }
                        }
                    },
                };
            }
        })
    }

    // pub async fn send(&mut self, msg: String) {
    //     self.rtc_server.send().await
    // }
}
