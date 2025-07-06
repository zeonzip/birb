use server::server::ApiServer;

mod crypto;
mod database;
mod datastore;
mod server;
mod host;

#[tokio::main]
async fn main() {
    // temporary expects
    let server = ApiServer::connect()
        .await
        .expect("Fatal error during initialization");

    server.run().await.expect("Fatal error during runtime")
}
