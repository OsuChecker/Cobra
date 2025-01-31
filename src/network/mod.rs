pub mod smol_hyper;

use http_body_util::Full;

use std::net::TcpListener;

use crate::{gosu_structs::GosuValues, structs::{Arm, Clients, OutputValues, WsClient, WsKind}};

use self::smol_hyper::SmolIo;
use futures_util::sink::SinkExt;
use smol::{prelude::*, Async};

use async_tungstenite::{
    tungstenite::{handshake::derive_accept_key, protocol::Role, Message}, 
    WebSocketStream
};

use eyre::Result;
use hyper::{
    body::Bytes, header::{
        HeaderValue, CONNECTION, SEC_WEBSOCKET_ACCEPT, SEC_WEBSOCKET_KEY, UPGRADE}, server::conn::http1, service::service_fn, Request, Response, StatusCode
};


pub async fn handle_clients(values: Arm<OutputValues>) {
    let _span = tracy_client::span!("handle clients");

    let (serialized_rosu_values, serialized_gosu_values) = {
        let values_lock = values.lock().unwrap();

        let values = &*values_lock;

        let gosu_values: GosuValues = values.into();
    
        (
            serde_json::to_string(&values).unwrap(),
            serde_json::to_string(&gosu_values).unwrap(),
        )
    };
    println!("{:?}",serialized_gosu_values);


}

