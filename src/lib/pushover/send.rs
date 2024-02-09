use std::env;

use dotenvy::dotenv;
use reqwest::Url;
use crate::lib::pushover::message::NewMessage;

pub(crate) async fn send_message(message: NewMessage) -> Result<i32, String> {
    dotenv().ok();
    let is_ok = message.ok();

    if is_ok.is_err() {
        let error_message = format!("Error: {}", is_ok.unwrap_err());
        println!("Could not send notification: {} {:?}", error_message, message);
        return Err(error_message);
    }

    let token = env::var("PUSHOVER_TOKEN").expect("PUSHOVER_TOKEN must be set");
    let user = env::var("PUSHOVER_USER").expect("PUSHOVER_USER must be set");
    let client = reqwest::Client::new();

    println!("user: {}", user);

    let mut form = std::collections::HashMap::new();

    println!("Sending message {:#?}", message);

    form.insert("token", token);
    form.insert("user", user);
    form.insert("message", message.message);

    if message.title.is_some() {
        form.insert("title", message.title.unwrap());
    } else {
        form.insert("title", String::from("New message"));
    }

    if message.url.is_some() {
        form.insert("url", message.url.unwrap());
    }

    if message.url_title.is_some() {
        form.insert("url_title", message.url_title.unwrap());
    }

    if message.priority.is_some() {
        form.insert("priority", message.priority.unwrap().as_str());
    }

    if message.timestamp.is_some() {
        form.insert("timestamp", message.timestamp.unwrap().to_string());
    }

    if message.sound.is_some() {
        form.insert("sound", message.sound.unwrap().as_str());
    }

    if message.device.is_some() {
        form.insert("device", message.device.unwrap());
    }

    if message.html.is_some() {
        form.insert("html", (message.html.unwrap() as i32).to_string());
    }

    if message.monospace.is_some() {
        form.insert("monospace", (message.monospace.unwrap() as i32).to_string());
    }

    if message.retry.is_some() {
        form.insert("retry", message.retry.unwrap().to_string());
    }

    if message.expire.is_some() {
        form.insert("expire", message.expire.unwrap().to_string());
    }

    if message.callback.is_some() {
        form.insert("callback", message.callback.unwrap());
    }

    println!("Form: {:#?}", form);

    let url = Url::parse("https://api.pushover.net/1/messages.json").expect("Invalid URL");

    let response = client.post(url.as_str()).form(&form).send();

    let response = match response.await {
        Ok(r) => {
            if r.status().is_success() {
                println!("Message sent");
                Ok(200)
            } else {
                println!("Not success: {} {:?}", r.status(), r.text().await);
                Err(String::from("Error sending message"))
            }
        }
        Err(e) => {
            println!("Could not send: {}", e);
            Err(String::from("Error sending message"))
        }
    };

    response
}

