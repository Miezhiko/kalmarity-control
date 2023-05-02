use env_logger::Env;

use std::io;
use std::time::Duration;

use clap::{App, Arg};

use log::info; // , warn, error };

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

async fn produce(_cmd: &str, chan: u32, usr: u32, msg: u32, text: &str) {
  let producer: &FutureProducer = &ClientConfig::new()
    .set("bootstrap.servers", "localhost:9092")
    .set("message.timeout.ms", "5000")
    .create()
    .expect("Producer creation error");

  let future = async move {
    let k_key = format!("{chan}|{usr}|{msg}");
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

let matches = App::new("kalmarity-control")
  .version("0.1.0")
  .about("A simple kalmarity control util")
  .arg(
    Arg::new("COMMAND")
        .help("The command to execute")
        .required(true)
        .index(1),
  )
  .arg(
    Arg::new("CHANID")
        .help("The channel ID")
        .required(true)
        .index(2),
  )
  .arg(
    Arg::new("USRID")
        .help("The user ID")
        .required(false)
        .index(3),
  )
  .arg(
    Arg::new("MSGID")
        .help("The message ID")
        .required(false)
        .index(4),
  )
  .get_matches();

  let cmd         = matches.value_of("COMMAND").unwrap();
  let chan : u32  = matches.value_of("CHANID").unwrap().parse().unwrap();
  let usr : u32   = matches.value_of("USRID").unwrap_or("0").parse().unwrap();
  let msg : u32   = matches.value_of("MSGID").unwrap_or("0").parse().unwrap();

  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_)       => produce(cmd, chan, usr, msg, input.trim()).await,
    Err(error)  => println!("Error: {}", error)
  }

  Ok(())
}
