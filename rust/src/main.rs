use axum::{extract::Query, routing::get, Router};
use serde::{Deserialize, Serialize};
use std::fs;
use std::{env, fs::File, io::prelude::*};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(json_handler));
    println!("Started rust server on port 3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct QueryPayload {
    q1: String,
    q2: String,
    q3: String,
    q4: String,
}
#[derive(Serialize)]
struct QueryResponse {
    query_param_1: String,
    query_param_2: String,
    query_param_3: String,
    query_param_4: String,
}
async fn json_handler(query_params: Query<QueryPayload>) -> String {
    let params = query_params.0;
    let body = QueryResponse {
        query_param_1: params.q1,
        query_param_2: params.q2,
        query_param_3: params.q3,
        query_param_4: params.q4,
    };
    let j = serde_json::to_vec(&body).expect("error with json to string");
    let id = Uuid::new_v4();
    let current_dir = env::current_dir().expect("error with current dir");
    let json_path = current_dir.join("json".to_string());
    let file_path = json_path.join(&id.to_string());
    let mut file = File::create(file_path.clone()).expect("error creating");
    let j_slice: &[u8] = &j;

    file.write_all(j_slice).expect("error writing");
    let mut file = File::open(file_path.to_owned()).expect("error openning");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("error turning into string");
    fs::remove_file(file_path).expect("error deleting file");
    content
}
