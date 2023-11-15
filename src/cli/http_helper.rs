use crate::cli::command_exit::CommandExit;
use crate::cli::spinner::PendulumsSpinner;
use reqwest::{Client, RequestBuilder, Response, StatusCode};
use reqwest_cookie_store::CookieStoreMutex;
use std::fs::{create_dir_all, remove_file, File};
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Arc;

pub struct HttpHelper {
  cookie_path: PathBuf,
  pub http_client: Client,
  pub cookie_store: Arc<CookieStoreMutex>,
}

impl HttpHelper {
  pub fn build() -> HttpHelper {
    let mut cookie_path: PathBuf;
    cookie_path = match dirs::data_local_dir() {
      Some(path) => path,
      None => {
        panic!("Can not access local directory!")
      }
    };
    cookie_path = cookie_path.join("pendulums").join("cli");
    let _ = create_dir_all(&cookie_path);

    cookie_path = cookie_path.join("cookies.json");

    let cookie_store = {
      if let Ok(file) = File::open(&cookie_path).map(BufReader::new) {
        reqwest_cookie_store::CookieStoreMutex::new(
          reqwest_cookie_store::CookieStore::load_json(file).unwrap(),
        )
      } else {
        reqwest_cookie_store::CookieStoreMutex::new(reqwest_cookie_store::CookieStore::new(None))
      }
    };
    let cookie_store = &Arc::new(cookie_store).to_owned();
    HttpHelper {
      cookie_path,
      cookie_store: cookie_store.clone(),
      http_client: reqwest::Client::builder()
        .cookie_provider(cookie_store.clone())
        .build()
        .unwrap(),
    }
  }

  pub fn store_auth_cookie(&self) {
    let mut writer = File::create(&self.cookie_path)
      .map(std::io::BufWriter::new)
      .unwrap();
    self
      .cookie_store
      .lock()
      .unwrap()
      .save_json(&mut writer)
      .unwrap();
  }

  pub fn remove_auth_cookie(&self) -> Result<(), std::io::Error> {
    return remove_file(&self.cookie_path);
  }

  pub async fn request(&self, request: RequestBuilder) -> Result<Response, CommandExit> {
    let mut sp = PendulumsSpinner::start();
    let result = request.send().await;
    sp.stop();

    if result.is_err() {
      return Err(CommandExit::Error(String::from("No internet connection!")));
    }

    let res = result.unwrap();
    return match res.status() {
      StatusCode::SERVICE_UNAVAILABLE => match res.text().await {
        Ok(_) => Err(CommandExit::Error(String::from(
          "You have reached the authentication limits, please try in a few minutes!",
        ))),
        Err(_e) => Err(CommandExit::Error(String::from("Failed to sign in"))),
      },
      StatusCode::FORBIDDEN => match res.text().await {
        Ok(_) => Err(CommandExit::Error(String::from(
          "You need to sign in first!",
        ))),
        Err(_e) => Err(CommandExit::Error(String::from("Failed to sign in"))),
      },
      _ => Ok(res),
    };
  }
}
