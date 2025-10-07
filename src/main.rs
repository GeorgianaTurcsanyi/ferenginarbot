use std::collections::HashMap;
use std::fs;
use axum::{routing::get, Json, Router};
use axum::http::StatusCode;
use axum::routing::post;
use serde_json::{Value, json, };
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().
        route("/rules", get(get_rules)).
        route("/rule", post(get_random_rule));

    println!("starting to listen on port 3000");
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}


async fn get_rules() -> Result<Json<Value>, StatusCode> {
    match fs::read_to_string("./rules") {
        Ok(rules) => {
            let mut rules_map: HashMap<&str, &str> = HashMap::new();
            for line in rules.lines().enumerate() {
                let rule: Vec<_> = line.1.split(":").collect();
                rules_map.insert(rule[0], rule[1].trim_start());
            }
            Ok(Json(json!({"rules": rules_map})))
        }
        Err(e) => {
            eprintln!("error occurred opening rules file: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)

        }
    }
}

async fn get_random_rule(Json(input): Json<ZulipPayload>) -> Result<Json<Value>, StatusCode> {
    println!("received {:?}", input);
    match tokio::fs::read_to_string("./rules").await {
        Ok(rules) => {
            let mut rng = rand::rng();
            let rule: Vec<&str> = rules.lines().choose(&mut rng).unwrap().split(": ").collect();
            Ok(Json(json!({"rule": {rule[0]: rule[1]}})))
        }
        Err(e) => {
                eprintln!("error occurred opening rules file: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)

        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
struct ZulipMessage {
    pub avatar_url: String,
    pub client: String,
    pub content: String,
    pub display_recipient: Value,
    pub id: i64,
    pub is_me_message: bool,
    pub reactions: Value,
    pub recipient_id: i64,
    pub rendered_content: String,
    pub sender_email: String,
    pub sender_full_name: String,
    pub sender_id: i64,
    pub sender_realm_str: String,
    pub stream_id: i64,
    pub subject: String,
    pub submessages: Value,
    pub timestamp: i64,
    pub topic_links: Value,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ZulipPayload {
    pub bot_email: String,
    pub bot_full_name: String,
    pub data: String,
    pub message: ZulipMessage,
    pub token: String,
    pub trigger: String,
}