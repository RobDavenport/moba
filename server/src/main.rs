use std::collections::HashMap;
use std::thread;

use hyper::{
    header::{self, HeaderValue},
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Error, Method, Response, Server, StatusCode,
};

use webrtc_unreliable::Server as RtcServer;
use ws::*;

mod engine;
use crate::engine::network::client_factory::ClientFactory;
use engine::engine_builder::*;

const WEB_RTC_LISTEN: &str = "192.168.1.150:8001";
const WEB_RTC_PUBLIC: &str = "192.168.1.150:8001";
const HYPER_API_ADDR: &str = "192.168.1.150:8001";

const IP_ADDRESS: &str = "192.168.1.150:8000";
const TICKS_PER_SECOND: u8 = 30;

#[tokio::main]
async fn main() {
    let client_factory = build_engine(TICKS_PER_SECOND);

    let ws_thread = thread::spawn(|| {
        let ws_server = WebSocket::<ClientFactory>::new(client_factory).unwrap();
        println!("WS server listening at: {}", IP_ADDRESS);
        ws_server.listen(IP_ADDRESS).unwrap();
    });

    // ws_thread.join().unwrap();

    //TODO: WebRTC Server
    let mut rtc_server = tokio::spawn(RtcServer::new(
        WEB_RTC_LISTEN.parse().unwrap(),
        WEB_RTC_PUBLIC.parse().unwrap(),
    ))
    .await
    .unwrap()
    .expect("rtc server boom");
    let session_endpoint = rtc_server.session_endpoint();

    let make_svc = make_service_fn(move |addr_stream: &AddrStream| {
        let session_endpoint = session_endpoint.clone();
        let remote_addr = addr_stream.remote_addr();
        async move {
            Ok::<_, Error>(service_fn(move |req| {
                let mut session_endpoint = session_endpoint.clone();
                async move {
                    if req.uri().path() == "/sdp" && req.method() == Method::POST {
                        println!("WebRTC session request from {}", remote_addr);
                        match session_endpoint.http_session_request(req.into_body()).await {
                            Ok(mut resp) => {
                                resp.headers_mut().insert(
                                    header::ACCESS_CONTROL_ALLOW_ORIGIN,
                                    HeaderValue::from_static("*"),
                                );
                                Ok(resp.map(Body::from))
                            }
                            Err(err) => Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("error: {}", err))),
                        }
                    } else {
                        Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::from("not found"))
                    }
                }
            }))
        }
    });

    tokio::spawn(async move {
        println!("Http SDP requests at: {}", HYPER_API_ADDR);
        Server::bind(&HYPER_API_ADDR.parse().unwrap())
            .serve(make_svc)
            .await
            .expect("HTTP session server has died");
    });

    let mut message_buf = vec![0; 0x10000];
    let mut counter: u64 = 0;
    tokio::spawn(async move {
        loop {
            match rtc_server.recv(&mut message_buf).await {
                Ok(received) => {
                    let msg = &message_buf[0..received.message_len];
                    println!("sender: {}", &received.remote_addr);
                    println!("got message: {:?}", msg);
                }
                Err(err) => {
                    println!("couldnt recv! {}", err);
                }
            }
        }
    })
    .await
    .unwrap();
}
