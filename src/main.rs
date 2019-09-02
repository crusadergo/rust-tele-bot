use dotenv::dotenv;
use std::env;

fn test() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();

  let string = reqwest::get("https://api.telegram.org/bot{$API_KEY}/getMe")?.text()?;
  println!("{:#?}", string);
  Ok(())
}

fn main() {
  test();
  print!("yo")
}

