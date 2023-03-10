use axum::{extract::State, routing::get, Router};
#[allow(unused_imports)]
use entities::author::{self, AuthorToPost1, AuthorToPost2};
use sea_orm::{
    prelude::Uuid, Database, DatabaseConnection, EntityTrait, ModelTrait,
};

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
    let author_id = Uuid::parse_str("a42a2ab9-682a-4ed0-91ad-cf7bafc27c86").unwrap();

    let author = author::Entity::find_by_id(author_id)
        .one(&state.conn)
        .await
        .unwrap()
        .unwrap();

    let posts1 = author
        .find_linked(AuthorToPost2)
        .all(&state.conn)
        .await
        .unwrap();

    println!("{posts1:#?}");

    "hello world"
}
