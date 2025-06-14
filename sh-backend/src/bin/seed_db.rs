use anyhow::Result;
use clap::Parser;
use log::debug;
use sqlx::sqlite::SqlitePool;

#[path = "../models.rs"] // Adjust path if your models are elsewhere
mod models;
#[path = "../seed.rs"] // Adjust path if your seed function is elsewhere
mod seed;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    extend: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    env_logger::init();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set");

    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to SQLite, check out the README for setup instructions");

    let args = Cli::parse();

    let should_extend = args.extend;
    debug!(
        "Running the DB seed command with should_extend: {}",
        should_extend
    );
    seed::seed_db(&pool, should_extend).await?;

    Ok(())
}
