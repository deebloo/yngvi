mod program;

use crate::program::{find_reader, find_writer, var};
use dotenv::dotenv;
use yngvi::core::Station;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let source = var("SRC").unwrap_or("ACURITE_DISPLAY".to_string());
    let dest = var("DEST").unwrap_or("STDOUT".to_string());

    let mut station = Station::new();
    let reader = find_reader(&source);
    let mut writer = find_writer(&dest);

    println!(
        "Starting weather program. Reading from {} and writing to {}",
        source, dest
    );

    let res = station.start(reader, &mut writer).await;

    if res.is_ok() {
        println!("Station no longer recieving readings. Shutting down");
    }
}
