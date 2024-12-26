use std::{fs::File, str::FromStr, sync::Arc, thread};

use chrono::Utc;
use cron::Schedule;
use database::DIM;
use log::{debug, error, info, trace};
use reqwest::cookie::Jar;
use serde::Deserialize;
use serde_json::json;

mod database;

#[derive(Deserialize, Debug, Clone)]
struct Targets {
    url: String,
    name: String,
}

#[derive(Deserialize)]
struct Telegram {
    bot: String,
    chat: String,
}

#[derive(Deserialize)]
struct Notification {
    telegram: Option<Telegram>,
}

#[derive(Deserialize)]
struct Config {
    cron: String,
    targets: Vec<Targets>,
    notification: Option<Notification>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut db = DIM::<String, bool>::new();
    let file = File::open("config.json")
        .unwrap_or_else(|err| panic!("Couldn't open the config file\n{}", err));

    let config: Config = serde_json::from_reader(&file)
        .unwrap_or_else(|err| panic!("Config file is malformed\n{}", err));

    debug!("Cron job: {}", config.cron.as_str());

    let schedule = Schedule::from_str(config.cron.as_str()).expect("Failed to parse schedule");
    let notification = config.notification;

    loop {
        let now = Utc::now();
        if let Some(next) = schedule.upcoming(Utc).take(1).next() {
            let until_next = next - now;
            debug!("Next execution at: {}", next);
            thread::sleep(until_next.to_std().unwrap());
            debug!("Checking targets at {}", now);
            check_targets(&config.targets, &mut db, &notification).await;
        }
    }
}

async fn check_targets(
    targets: &[Targets],
    db: &mut DIM<String, bool>,
    notification: &Option<Notification>,
) {
    let jar = Arc::new(Jar::default());
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .cookie_provider(jar)
        .build()
        .unwrap();

    for target in targets {
        let key = format!("{}_status", target.name);
        let current_status = db.get(&key).unwrap_or(&true).clone();
        debug!("Target {} current status: {}", target.name, current_status);

        let response = client
            .get(&target.url)
            .header("Connection", "keep-alive")
            .send()
            .await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    trace!(
                        "Target {} response text: {}",
                        target.name,
                        res.text()
                            .await
                            .unwrap_or("Error on unwrap response".to_string())
                    );
                    if current_status != true {
                        let message = format!("The service {} is back up", target.name);
                        info!("{}", message);
                        send_notification(message, notification).await;
                    }
                    db.set(key, true);
                    info!("Target {} new status: true", target.name);
                } else {
                    let message = format!(
                        "The service {} check got status: {}",
                        target.name,
                        res.status()
                    );
                    error!("{}", message);
                    send_notification(message, notification).await;
                    db.set(key, false);
                    info!("Target {} new status: false", target.name);
                }
            }
            Err(err) => {
                let message = format!("Failed to make the request to {}: {}", target.name, err);
                error!("{}", message);
                send_notification(message, notification).await;
            }
        }
    }
}

async fn send_notification(message: String, notification: &Option<Notification>) {
    if let Some(notification) = notification {
        if let Some(telegram) = &notification.telegram {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("Content-Type", "application/json".parse().unwrap());

            let body = json!({
                "chat_id": &telegram.chat,
                "text": message,
                "disable_notification": true
            }
            );

            let url = format!("https://api.telegram.org/bot{}/sendMessage", &telegram.bot);

            let _ = reqwest::Client::new().post(url).json(&body).send().await;
        }
    }
}
