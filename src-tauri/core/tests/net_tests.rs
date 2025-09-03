use spacebar_core::{Gateway, GatewayEvent, RestClient, RestError, RouteSettings};
use tokio::sync::mpsc;
use url::Url;

#[tokio::test]
async fn gateway_dispatches_events() {
    use futures_util::{SinkExt, StreamExt};
    use serde_json::json;
    use tokio::net::TcpListener;
    use tokio_tungstenite::{accept_async, tungstenite::Message};
    use tokio::time::{sleep, Duration};

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let url = Url::parse(&format!("ws://{}", addr)).unwrap();
    let (tx, mut rx) = mpsc::unbounded_channel();

    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut ws = accept_async(stream).await.unwrap();

        // receive identify
        if let Some(Ok(Message::Text(_))) = ws.next().await {}

        // send hello with short heartbeat interval
        let hello = json!({"op":10,"d":{"heartbeat_interval":50}});
        ws.send(Message::Text(hello.to_string())).await.unwrap();

        // expect heartbeat
        if let Some(Ok(Message::Text(_))) = ws.next().await {}

        // send dispatch event
        let dispatch = json!({"op":0,"t":"TEST","s":1,"d":{"value":1}});
        ws.send(Message::Text(dispatch.to_string())).await.unwrap();

        sleep(Duration::from_millis(100)).await;
        ws.close(None).await.unwrap();
    });

    let mut gateway = Gateway::new(url, "token".into(), tx);
    let client = tokio::spawn(async move {
        gateway.start().await;
    });

    match rx.recv().await {
        Some(GatewayEvent::Dispatch { event, data }) => {
            assert_eq!(event, "TEST");
            assert_eq!(data["value"], 1);
        }
        other => panic!("unexpected event: {:?}", other),
    }

    client.abort();
    server.await.unwrap();
}

#[tokio::test]
async fn rest_client_parses_and_errors() {
    use httpmock::MockServer;
    use serde::Deserialize;
    use serde_json::json;

    let server = MockServer::start_async().await;

    // success response
    server.mock(|when, then| {
        when.method("GET").path("/api/test");
        then.status(200).json_body(json!({"value": 5}));
    });

    // error response
    server.mock(|when, then| {
        when.method("GET").path("/api/error");
        then.status(400).json_body(json!({"message": "bad"}));
    });

    let mut routes = RouteSettings::default();
    routes.api = server.url("/api");
    let client = RestClient::new(routes);

    #[derive(Deserialize, Debug)]
    struct Resp { value: i32 }

    let r: Resp = client.get("/test", &[]).await.unwrap();
    assert_eq!(r.value, 5);

    let err = client.get::<Resp>("/error", &[]).await.unwrap_err();
    match err {
        RestError::Api(msg) => assert_eq!(msg, "bad"),
        _ => panic!("unexpected error"),
    }
}

