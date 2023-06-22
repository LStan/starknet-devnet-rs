use std::net::SocketAddr;

use axum::routing::get;
use axum::routing::post;
use server::builder::StarknetDevnetServer;
use server::ServerConfig;

use crate::api;
use crate::api::http;
use crate::api::http::HttpApiHandler;
use crate::api::json_rpc::JsonRpcHandler;
use crate::api::Api;

/// Configures an [axum::Server] that handles related JSON-RPC calls and WEB API calls via HTTP
pub fn serve_http_api_json_rpc(addr: SocketAddr, config: ServerConfig) -> StarknetDevnetServer {
    let api = Api::new();
    let http = api::http::HttpApiHandler { api: api.clone() };
    let json_rpc = api::json_rpc::JsonRpcHandler { api };

    server::builder::Builder::<JsonRpcHandler, HttpApiHandler>::new(addr)
        .set_config(config)
        .json_rpc_route("/rpc", json_rpc)
        .http_api_route("/is_alive", get(http::is_alive))
        .http_api_route("/dump", post(http::dump))
        .http_api_route("/load", post(http::load))
        .http_api_route(
            "/postman/load_l1_messaging_contract",
            post(http::postman_load),
        )
        .http_api_route("/postman/flush", post(http::postman_flush))
        .http_api_route(
            "/postman/send_message_to_l2",
            post(http::postman_send_message_to_l2),
        )
        .http_api_route(
            "/postman/consume_message_from_l2",
            post(http::postman_consume_message_from_l2),
        )
        .http_api_route("/create_block", post(http::create_block))
        .http_api_route("/abort_blocks", post(http::abort_blocks))
        .http_api_route("/restart", post(http::retart))
        .http_api_route("/set_time", post(http::set_time))
        .http_api_route("/increase_time", post(http::increase_time))
        .http_api_route("/predeployed_accounts", get(http::predeployed_accounts))
        .http_api_route("/get_code", get(http::get_contract_code))
        .http_api_route("/account_balance", get(http::get_account_balance))
        .http_api_route("/fee_token", get(http::get_fee_token))
        .http_api_route("/mint", post(http::mint))
        .http_api_route("/fork_status", get(http::get_fork_status))
        .set_http_api_handler(http)
        .build()
}
