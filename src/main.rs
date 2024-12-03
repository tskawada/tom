use dirs_next;
use std::fs;
use std::process::Command;
use std::env;
use serde::Deserialize;
use reqwest::Client;

#[derive(Deserialize)]
struct Config {
    pushover_app_token: String,
    pushover_user_key: String,
}

#[tokio::main]
async fn main() {
    // 引数を取得
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: tom \"<command>\"");
        std::process::exit(1);
    }
    let command = &args[1];

    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    println!("Executing command: {}", command);
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");

    println!("Command finished with status: {:?}", output.status);
    println!("Standard Output: {}", String::from_utf8_lossy(&output.stdout));
    println!("Standard Error: {}", String::from_utf8_lossy(&output.stderr));

    if let Err(e) = send_push_notification(output.status.success(), &config).await {
        eprintln!("Failed to send push notification: {}", e);
    }
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let home_dir = dirs_next::home_dir().ok_or("Failed to find home directory")?;
    let config_path = home_dir.join(".tomrc");

    let config_content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

async fn send_push_notification(success: bool, config: &Config) -> Result<(), reqwest::Error> {
    let client = Client::new();

    #[derive(serde::Serialize)]
    struct Notification {
        token: String,
        user: String,
        message: String,
    }

    let notification = Notification {
        token: config.pushover_app_token.clone(),
        user: config.pushover_user_key.clone(),
        message: if success {
            "Command completed successfully!".to_string()
        } else {
            "Command failed.".to_string()
        },
    };

    let res = client
        .post("https://api.pushover.net/1/messages.json")
        .json(&notification)
        .send()
        .await?;

    if res.status().is_success() {
        println!("Push notification sent!");
    } else {
        println!("Failed to send push notification. Response: {:?}", res.text().await?);
    }
    Ok(())
}
