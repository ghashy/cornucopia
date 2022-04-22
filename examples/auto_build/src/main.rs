use deadpool_postgres::{Config, Runtime};
use tokio_postgres::NoTls;

// Since we generated the cornucopia file inside our project, you can simply
// import the modules as you usually would, but you could also use something
// like `include_str` if you wanted to.
mod cornucopia;

// Add more migrations and queries, rebuild the crate,
// and observe how your cornucopia modules are regenerated!
#[tokio::main]
async fn main() {
    // Take a look at the generated `cornucopia.rs` file if you want to
    // see what it looks like under the hood.
    use crate::cornucopia::queries::module_1::*;

    // Connection pool configuration
    // This has nothing to do with cornucopia, please look at
    // `tokio_postgres` and `deadpool_postgres` for details
    let mut cfg = Config::new();
    cfg.user = Some(String::from("postgres"));
    cfg.password = Some(String::from("postgres"));
    cfg.host = Some(String::from("127.0.0.1"));
    cfg.port = Some(5432);
    cfg.dbname = Some(String::from("postgres"));
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let client = pool.get().await.unwrap();

    example_query1(&client).await.unwrap();
}