use dotenv::dotenv;
use std::env;
use hyper::Client;

const TELEGRAM_API_URL_DEFAULT: &str = "https://api.telegram.org/";
// const API_TOKEN: String = env::var("API_KEY").expect("API_KEY not set");


fn telegram_api_url() -> String {
  dotenv().ok();

  match env::var("API_URL") {
      Ok(url) => url,
      Err(_) => String::from(TELEGRAM_API_URL_DEFAULT),
  }
}

fn get_me() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();

  let url = format!("{}bot{}/getMe", telegram_api_url(), env::var("API_KEY").expect("API_KEY not set"));
  let res = reqwest::get(&url)?.text()?;
  
  println!("{:#?}", res);

  Ok(())
}

fn send_message() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();

  let url = format!("{}bot{}/sendMessage?chat_id=-244044622&text=😌", telegram_api_url(), env::var("API_KEY").expect("API_KEY not set"));
  let res = reqwest::get(&url)?.text()?;

  println!("{:#?}/wow", res);
  Ok(())
}

fn main() {
  // get_me();
  send_message();
  print!("yo")
}

