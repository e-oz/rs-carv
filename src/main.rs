use std::collections::HashMap;
use structopt::StructOpt;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use reqwest::header;
use std::cmp::Reverse;

#[derive(Debug, StructOpt)]
#[structopt(name = "args")]
struct Opt {
  user: String,

  #[structopt(long)]
  token: Option<String>,

  #[structopt(parse(from_os_str), long = "file")]
  output_file: Option<PathBuf>,

  #[structopt(long = "format")]
  out_format: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
  pub name: String,
  pub html_url: String,
  pub description: Option<String>,
  pub stargazers_count: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args: Opt = Opt::from_args();
  let username = args.user;

  let mut headers = header::HeaderMap::new();
  if let Some(token) = args.token {
    if let Ok(h) = header::HeaderValue::from_str(&format!("token {}", token)) {
      headers.insert(header::AUTHORIZATION, h);
    }
  }
  headers.insert(header::USER_AGENT, header::HeaderValue::from_static("Safari"));

  let client = reqwest::blocking::Client::builder()
    .default_headers(headers)
    .build()?;
  let r = client
    .get(&format!("https://api.github.com/users/{}/repos", username))
    .header(header::CONTENT_TYPE, "application/json")
    .send()?;
  let mut repos:Vec<Repo> = r.json::<Vec<Repo>>()?;
  repos.sort_by_key(|d| Reverse(d.stargazers_count));
  let mut iter = repos.into_iter();

  for _ in 0..9 {
    if let Some(repo) = iter.next() {
      println!("Repo: {}", repo.name);
      println!("URL: {}", repo.html_url);
      if let Some(descr) = repo.description {
        println!("Description: {}", descr);
      }
      println!("Stars: {}\n", repo.stargazers_count);
    }
  }
  Ok(())
}