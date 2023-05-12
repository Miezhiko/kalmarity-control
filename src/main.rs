use env_logger::Env;

use std::io;
use std::time::Duration;

use log::{ info, error };

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

use clap::Parser;

/// Kalmarity control util
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// command to execute
  #[arg(short, long)]
  command: Option<String>,

  /// channel id
  #[arg(short, long, default_value_t = 0)]
  channel: u64,

  /// user id to mention
  #[arg(short, long, default_value_t = 0)]
  user: u64,

  /// msg id to reply
  #[arg(short, long, default_value_t = 0)]
  msg: u64
}

async fn produce(args: Args, text: &str) {
  let producer: &FutureProducer = &ClientConfig::new()
    .set("bootstrap.servers", "localhost:9092")
    .set("message.timeout.ms", "5000")
    .create()
    .expect("Producer creation error");

  let future = async move {
    let k_key = format!("{}|{}|{}"
                       , args.channel
                       , args.user
                       , args.msg);
    let delivery_status = producer
      .send(
        FutureRecord::to("Kalmarity")
          .payload(text)
          .key(&k_key),
        Duration::from_secs(0),
      )
      .await;

    // This will be executed when the result is received.
    info!("Delivery status for message received");
    delivery_status
  };

  let _ = future.await;
}

#[tokio::main(worker_threads=8)]
async fn main() -> anyhow::Result<()> {
  env_logger::Builder
            ::from_env(
              Env::default().default_filter_or("info")
            ).init();

  let args = Args::parse();

  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_)       => produce(args, input.trim()).await,
    Err(error)  => error!("Error: {}", error)
  }

  Ok(())
}
