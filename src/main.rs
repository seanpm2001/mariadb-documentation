use mariadb_archive_server::run;

#[tokio::main]
pub async fn main() {
    run().await;
}
