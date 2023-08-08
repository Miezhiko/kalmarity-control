use env_logger::Env;

use std::{
  io,
  time::Duration
};

use log::error;

use rdkafka::{
  config::ClientConfig,
  producer::{FutureProducer, FutureRecord}
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(long)]
  command: Option<String>,

  #[arg(short, long, default_value_t = 0)]
  channel: u64,

  #[arg(short, long, default_value_t = 0)]
  user: u64,

  #[arg(short, long, default_value_t = 0)]
  msg: u64
}

async fn produce(args: &Args, text: &str) {
  let producer: &FutureProducer = &ClientConfig::new()
    .set("bootstrap.servers", "localhost:9092")
    .set("message.timeout.ms", "5000")
    .create()
    .expect("Producer creation error");
  let _delivery_status = producer
    .send(
      FutureRecord::to("Kalmarity")
        .payload(text)
        .key( &format!("{}|{}|{}"
            , args.channel
            , args.user
            , args.msg)),
      Duration::from_secs(0),
    ).await;
}

#[tokio::main(worker_threads=8)]
async fn main() -> anyhow::Result<()> {
  env_logger::Builder
            ::from_env(
              Env::default().default_filter_or("info")
            ).init();

  let args = Args::parse();

  let mut input = String::new();

  tokio::spawn( async move {
    loop {
      match io::stdin().read_line(&mut input) {
        Ok(_)       => produce(&args, input.trim()).await,
        Err(error)  => error!("Error: {}", error)
      }
    }
  }).await?;

  tokio::signal::ctrl_c().await?;

  Ok(())
}
