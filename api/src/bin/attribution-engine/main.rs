mod background_worker;
mod server;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "attribution-engine")]
enum Command {
    Server,
    BackgroundWorker,
}

#[tokio::main]
async fn main() {
    use clap::Parser;

    match Command::parse() {
        Command::BackgroundWorker => background_worker::run().await.unwrap(),
        Command::Server => server::run().await.unwrap(),
    }
}
