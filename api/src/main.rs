use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use surrealdb::Surreal;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root};
mod models;
use models::{CreateUser, User};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(hello))
        .route("/get_users", get(get_users))
        .route("/create_user", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "Hello, World!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> Result<StatusCode, (StatusCode, String)> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error connecting to database: {:?}", e),
        )
    })?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            format!("Error signing in: {:?}", e),
        )
    })?;

    db.use_ns("test").use_db("test").await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error setting namespace and database: {:?}", e),
        )
    })?;

    let user: Vec<User> = db
        .create("user")
        .content(CreateUser {
            name: payload.name,
            email: payload.email,
        })
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error creating user: {:?}", err),
            )
        })?;
    tracing::info!("Created user: {:?}", user);

    Ok(StatusCode::CREATED)
}

async fn get_users() -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error connecting to database: {:?}", e),
        )
    })?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            format!("Error signing in: {:?}", e),
        )
    })?;

    db.use_ns("test").use_db("test").await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error setting namespace and database: {:?}", e),
        )
    })?;
    let users: Vec<User> = db.select("user").await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error getting users: {:?}", e),
        )
    })?;
    tracing::info!("Got users: {:?}", users);
    Ok(Json(users))
}
