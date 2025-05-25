use axum::{
    routing::post,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use uuid::Uuid;
use std::fs::{OpenOptions};
use std::io::{Read, Write};
use axum::response::IntoResponse;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Goal {
    id: String,
    title: String,
    description: Option<String>,
    amount: f64,
    months: u32,
}

#[derive(Deserialize)]
struct GoalRequest {
    title: String,
    description: Option<String>,
    amount: f64,
    months: u32,
}

 
async fn add_goal(Json(payload): Json<GoalRequest>) -> Json<Goal> {
    let goal = Goal {
        id: Uuid::new_v4().to_string(),
        title: payload.title,
        description: payload.description,
        amount: payload.amount,
        months: payload.months,
    };

    println!("Saving goal: {:?}", goal);

    // File path
    let file_path = "goals.json";

    // Read existing goals
    let mut existing = Vec::new();
    if let Ok(mut file) = OpenOptions::new().read(true).open(file_path) {
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap_or(0);
        if let Ok(parsed) = serde_json::from_str::<Vec<Goal>>(&content) {
            existing = parsed;
        }
    }

    // Add new goal
    existing.push(goal.clone());

    // Write back to file
    let json = serde_json::to_string_pretty(&existing).unwrap();
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(file_path).unwrap();
    file.write_all(json.as_bytes()).unwrap();

    Json(goal)
}


#[tokio::main]
async fn main() {
    // build our app
    let app = Router::new()
        .route("/goals", post(add_goal).get(get_goals));
        println!("Routes set up for /goals [GET, POST]");


    // bind using TcpListener
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Server running at http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

//Get Handler

async fn get_goals() -> impl IntoResponse {
    let file_path = "goals.json";

    let goals = if let Ok(mut file) = OpenOptions::new().read(true).open(file_path) {
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap_or(0);
        serde_json::from_str::<Vec<Goal>>(&content).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    };

    Json(goals)
}

