use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use tokio::{net::TcpListener, sync::Mutex};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_state = Arc::new(tokio::sync::Mutex::new(Vec::new()));

    let app = Router::new()
        // .route("/:name", get(handler)) // @Deprecated
        .route("/{name}", get(handler))
        .with_state(
            // axum::extract::State(app_state.clone()) -> State(previous_names): State<AppState>
            // app_state.clone()                       -> State(previous_names): AppState
            app_state.clone(),
        );

    // @Deprecated
    // let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    // axum::Server::bind(&addr)
    //     .serve(router)
    //     .await
    //     .unwrap()

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    println!("This line won't be printed.");

    Ok::<_, anyhow::Error>(()) // Just so we can use `await?` freely.
}

// State<> contains global data which is available for EVERY request.
type AppState = State<Arc<Mutex<Vec<String>>>>;

async fn handler(
    // `name` if of type String
    Path(name): Path<String>,
    // `previous_names` is of type Arc<Mutex<Vec<String>>>
    State(previous_names): AppState,
) -> Html<String> {
    let mut response = format!("<h1>Hello, {name}!</h1>");

    let mut past_names: tokio::sync::MutexGuard<'_, Vec<String>> = previous_names.lock().await;
    if !past_names.is_empty() {
        response += "Names we have seen:";
        response += "<ul>";
        past_names
            .iter()
            .for_each(|n| response += &format!("<li>{n}</li>"));
        response += "</ul>";
    }

    past_names.push(name);

    Html(response)
}
