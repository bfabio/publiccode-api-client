use clap::{Parser, Subcommand};
use reqwest::{Client, Method};
use serde::Deserialize;
use serde_json::Value;
use std::env;
use std::fmt;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CreatePublisher { data: String },
    CreateSoftware { data: String },
    UpdateSoftware { software_id: String, data: String },
    ListSoftware,
    ListPublishers { code_hostings: bool },
    ShowSoftware { software_id: String },
    ShowPublisher { publisher_id: String },
    Logs,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Publisher {
    id: String,
    alternative_id: Option<String>,
    description: String,
    email: Option<String>,
    active: bool,
}

impl fmt::Display for Publisher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/publishers/{} alternativeId={} description={} email={} active={}",
            API_BASE_URL,
            self.id,
            self.alternative_id.as_deref().unwrap_or(""),
            self.description,
            self.email.as_deref().unwrap_or(""),
            self.active,
        )
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Software {
    id: String,
    url: String,
    publiccode_yml: String,
    active: bool,
}

impl fmt::Display for Software {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/publishers/{} ({}) active={} publiccodeYml={}",
            API_BASE_URL, self.id, self.url, self.active, self.publiccode_yml,
        )
    }
}

const API_BASE_URL: &str = "https://api.developers.italia.it/v1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let bearer = env::var("API_BEARER_TOKEN").unwrap_or_default();
    let client = Client::new();

    match cli.command {
        Commands::CreatePublisher { data } => {
            let json_data: Value = serde_json::from_str(&data)?;
            let res = api_request(
                &client,
                "publishers",
                Method::POST,
                Some(json_data),
                &bearer,
            )
            .await?;
            println!("{res}");
        }
        Commands::CreateSoftware { data } => {
            let json_data: Value = serde_json::from_str(&data)?;
            let res =
                api_request(&client, "software", Method::POST, Some(json_data), &bearer).await?;
            println!("{res}");
        }
        Commands::UpdateSoftware { software_id, data } => {
            let json_data: Value = serde_json::from_str(&data)?;
            let res = api_request(
                &client,
                &format!("software/{software_id}"),
                Method::PATCH,
                Some(json_data),
                &bearer,
            )
            .await?;
            println!("{res}");
        }
        Commands::ListSoftware => {
            let items = get_paginated(&client, "software", &bearer).await?;
            for s in items {
                let s: Software = serde_json::from_value(s)?;
                println!("{s}");
            }
        }
        Commands::ListPublishers { code_hostings } => {
            let items = get_paginated(&client, "publishers", &bearer).await?;
            for p in items {
                if code_hostings {
                    if let Some(code_hostings) = p.get("codeHosting") {
                        for c in code_hostings.as_array().unwrap_or(&vec![]) {
                            println!("{c:#?}");
                        }
                    }
                } else {
                    let p: Publisher = serde_json::from_value(p)?;
                    println!("{p}");
                }
            }
        }
        Commands::ShowSoftware { software_id } => {
            let res = api_request(
                &client,
                &format!("software/{software_id}"),
                Method::GET,
                None,
                &bearer,
            )
            .await?;

            let s: Software = serde_json::from_value(res)?;
            println!("{s}");
        }
        Commands::ShowPublisher { publisher_id } => {
            let res = api_request(
                &client,
                &format!("publishers/{publisher_id}"),
                Method::GET,
                None,
                &bearer,
            )
            .await?;

            let p: Publisher = serde_json::from_value(res)?;
            println!("{p}");
        }
        Commands::Logs => {
            let items = get_paginated(&client, "logs", &bearer).await?;
            for l in items {
                println!("{l:#?}");
            }
        }
    }

    Ok(())
}

async fn api_request(
    client: &Client,
    resource: &str,
    method: Method,
    data: Option<Value>,
    bearer: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let url = format!("{API_BASE_URL}/{resource}");
    let mut req = client
        .request(method, &url)
        .header("Authorization", format!("Bearer {bearer}"))
        .header("Content-Type", "application/json");

    if let Some(body) = data {
        req = req.json(&body);
    }

    let res = req.send().await?;
    let res_json: Value = res.json().await?;

    Ok(res_json)
}

async fn get_paginated(
    client: &Client,
    resource: &str,
    bearer: &str,
) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let mut items = Vec::new();
    let mut next_page: Option<String> = None;

    loop {
        let url = match &next_page {
            Some(suffix) => format!("{API_BASE_URL}/{resource}?{suffix}"),
            None => format!("{API_BASE_URL}/{resource}?all=true"),
        };

        let res = client
            .get(&url)
            .header("Authorization", format!("Bearer {bearer}"))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let json: Value = res.json().await?;
        if let Some(data) = json.get("data") {
            if let Some(arr) = data.as_array() {
                items.extend(arr.clone());
            }
        }

        next_page = json
            .get("links")
            .and_then(|l| l.get("next"))
            .and_then(|s| s.as_str())
            .map(|s| s.trim_start_matches('?').to_string());

        if next_page.is_none() {
            break;
        }
    }

    Ok(items)
}
