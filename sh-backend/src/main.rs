mod models;
mod schema;

use async_graphql::{EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::routes;
use rocket::{State, response::content};
use schema::{AppSchema, SiteMutationRoot, SiteQueryRoot};
use sqlx::sqlite::SqlitePool;

#[rocket::get("/graphiql")]
async fn graphiql() -> content::RawHtml<String> {
    content::RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[rocket::get("/graphql?<query>")]
async fn graphql_query(schema: &State<AppSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<AppSchema>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

#[rocket::launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();

    env_logger::init();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set");

    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to SQLite, check out the README for setup instructions");

    let env = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "production".into());
    if env == "development" {
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run database migrations");
    }

    let schema = Schema::build(SiteQueryRoot, SiteMutationRoot, EmptySubscription)
        .data(pool.clone())
        .finish();

    rocket::build()
        .manage(pool)
        .manage(schema)
        .mount("/", routes![graphql_query, graphql_request, graphiql])
}
