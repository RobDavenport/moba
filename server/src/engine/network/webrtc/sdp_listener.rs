use hyper::{
  header::{self, HeaderValue},
  server::conn::AddrStream,
  service::{make_service_fn, service_fn},
  Body, Error, Method, Response, Server, StatusCode,
};

use webrtc_unreliable::SessionEndpoint;

pub async fn start_sdp_listener(
  sdp_addr: String,
  endpoint: SessionEndpoint,
) -> tokio::task::JoinHandle<()> {
  println!("start sdp listener");
  let make_svc = make_service_fn(move |addr_stream: &AddrStream| {
      let session_endpoint = endpoint.clone();
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
      println!("Http SDP requests at: {}", sdp_addr);
      Server::bind(&sdp_addr.parse().unwrap())
          .serve(make_svc)
          .await
          .expect("HTTP session server has died");
  })
}