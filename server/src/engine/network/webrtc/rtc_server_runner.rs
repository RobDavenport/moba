use bytes::Bytes;
use futures::stream::TryStreamExt;
use futures::{FutureExt, StreamExt};
use std::net::SocketAddrV4;
use warp::http::header;
use warp::reject::Rejection;
use warp::{Buf, Filter};

use webrtc_unreliable::Server as RtcServer;
use webrtc_unreliable::SessionEndpoint;

pub async fn start_rtc_server(listen_addr: String, public_addr: String) -> RtcServer {
    let rtc_server = tokio::spawn(RtcServer::new(
        listen_addr.parse().unwrap(),
        public_addr.parse().unwrap(),
    ))
    .await
    .unwrap()
    .expect("rtc server failed to start");

    rtc_server
}

pub async fn start_sdp_listener(
    sdp_addr: String,
    endpoint: SessionEndpoint,
) -> tokio::task::JoinHandle<()> {
    println!("start sdp listener");

    // let ws = warp::path("ws")
    //     .and(warp::ws())
    //     .map(|ws: warp::ws::Ws| {
    //         // println!("ws!");

    //         ws.on_upgrade(|websocket| {
    //             println!("Ws upgraded!");
    //             Ok("ws upgraded")
    //             // // Just echo all messages back...
    //             // let (tx, rx) = websocket.split();
    //             // rx.forward(tx).map(|result| {
    //             //     if let Err(e) = result {
    //             //         eprintln!("websocket error: {:?}", e);
    //             //     }
    //             // })
    //         })
    //     });

    tokio::spawn(async move {
        let sdp = warp::post()
            .and(warp::path("sdp"))
            .and(warp::body::stream())
            .and_then(move |data| {
                println!("Incomming SDP request...");
                let mut session_endpoint = endpoint.clone();
                let bytes = TryStreamExt::map_ok(data, |mut buf| Buf::to_bytes(&mut buf));
                async move {
                    match session_endpoint.http_session_request(bytes).await {
                        Ok(mut resp) => {
                            resp.headers_mut().insert(
                                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                                header::HeaderValue::from_static("*"),
                            );
                            Ok::<_, Rejection>(resp)
                        }
                        Err(err) => Err::<_, Rejection>(warp::reject()),
                    }
                }
            });

        // let ws = warp::path("ws")
        //     // The `ws()` filter will prepare the Websocket handshake.
        //     .and(warp::ws())
        //     .map(|ws: warp::ws::Ws| {
        //         // And then our closure will be called when it completes...
        //         ws.on_upgrade(|websocket| {
        //             // Just echo all messages back...
        //             let (sender, receiver) = websocket.split();
        //             receiver.forward(sender).map(|result| {
        //                 if let Err(e) = result {
        //                     eprintln!("websocket error: {:?}", e);
        //                 }
        //             })
        //         })
        //     });

        // let routes = ws.or(sdp);

        warp::serve(sdp)
            .run(sdp_addr.parse::<SocketAddrV4>().unwrap())
            .await
    })
}
