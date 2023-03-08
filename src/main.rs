use axum::{extract::State, routing::get, Router};
use entities::author;
use sea_orm::{prelude::Uuid, Database, DatabaseConnection, EntityTrait};

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

type AppStateExtract = State<AppState>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::connect("postgres://postgres:postgres@localhost:5432/seaorm").await?;
    let state = AppState { conn: db };

    let app = Router::new().route("/", get(test)).with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn test(State(state): AppStateExtract) -> &'static str {
    let author_id = Uuid::parse_str("0c8896b0-5b28-4ea1-b4d1-38921385e0fc").unwrap();

    let author = author::Entity::find_by_id(author_id)
        .one(&state.conn)
        .await
        .unwrap()
        .unwrap();

    println!("{author:#?}");

    "hello world"
}
