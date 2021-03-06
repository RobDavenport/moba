use std::net::SocketAddrV4;
use std::sync::atomic::{AtomicU32, Ordering};

use futures::{stream::TryStreamExt, SinkExt, StreamExt};
use tokio::sync::mpsc::Sender;
use warp::{filters::ws::Message, http::header, reject::Rejection, Buf, Filter};

use uuid::Uuid;
use webrtc_unreliable::{Server as RtcServer, SessionEndpoint};

use crate::engine::{
    components::player_controlled::PlayerId,
    messaging::messages::{OutMessage, WSClientMessage},
    network::client_data::ClientData,
};

static NEXT_USER_ID: AtomicU32 = AtomicU32::new(0);

pub async fn start_rtc_server(listen_addr: String, public_addr: String) -> RtcServer {
    tokio::spawn(RtcServer::new(
        listen_addr.parse().unwrap(),
        public_addr.parse().unwrap(),
    ))
    .await
    .unwrap()
    .expect("rtc server failed to start")
}

pub async fn start_service(
    sdp_addr: String,
    endpoint: SessionEndpoint,
    manager_out: Sender<WSClientMessage>,
) -> tokio::task::JoinHandle<()> {
    println!("Web service starting.");

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
                        Err(_err) => Err::<_, Rejection>(warp::reject()),
                    }
                }
            });

        let ws = warp::path("ws")
            .map(move || manager_out.clone())
            .and(warp::ws())
            .map(|manager_out, ws: warp::ws::Ws| {
                println!("Ws upgrade request");
                ws.on_upgrade(|websocket| ws_connected(websocket, manager_out))
            });

        let routes = ws.or(sdp);

        warp::serve(routes)
            .run(sdp_addr.parse::<SocketAddrV4>().unwrap())
            .await
    })
}

async fn ws_connected(ws: warp::ws::WebSocket, mut manager_out: Sender<WSClientMessage>) {
    let my_id = PlayerId(NEXT_USER_ID.fetch_add(1, Ordering::Relaxed));
    let uuid = Uuid::new_v4().to_string();

    println!("Ws connected. User id: {} with uuid: {}", my_id, &uuid);

    let (mut sender, mut receiver) = ws.split();
    match sender
        .send(Message::binary(
            OutMessage::VerifyUuid(uuid.clone()).into_proto_bytes(),
        ))
        .await
    {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    };

    manager_out
        .try_send(WSClientMessage::Connected(
            my_id,
            ClientData {
                ws_client_out: sender,
                socket_addr: None,
                socket_uuid: uuid,
            },
        ))
        .unwrap();

    while let Some(result) = receiver.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                println!("Websocket error! {}", e);
                break;
            }
        };
        manager_out
            .try_send(WSClientMessage::Packet(my_id, msg.into_bytes()))
            .unwrap();
    }

    //Client disconnected
    manager_out
        .try_send(WSClientMessage::Disconnected(my_id))
        .unwrap();
}
