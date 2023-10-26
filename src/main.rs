use clap::Parser;

#[derive(Parser)]
struct Args {
    /// The port in which the server will open
    #[arg(short, long, default_value = "8000")]
    port: u16,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let args = Args::parse();

    backend::server(args.port).await;
}
