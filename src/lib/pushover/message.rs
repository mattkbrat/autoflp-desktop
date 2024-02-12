use crate::lib::pushover::priority::Priority;
use crate::lib::pushover::sounds::Sound;

pub enum TransactionType {
    New,
    Delete
}

#[derive(Debug)]
pub struct NewMessage {
    pub token: String,
    pub user: String,
    pub message: String,
    pub title: Option<String>,
    pub url: Option<String>,
    pub url_title: Option<String>,
    pub priority: Option<Priority>,
    pub timestamp: Option<i64>,
    pub sound: Option<Sound>,
    pub device: Option<String>,
    pub html: Option<bool>,
    pub monospace: Option<bool>,
    pub retry: Option<i64>,
    pub expire: Option<i64>,
    pub callback: Option<String>,
}

impl NewMessage {
    pub fn new(token: String, user: String, message: String) -> NewMessage {
        NewMessage {
            token,
            user,
            message,
            title: None,
            url: None,
            url_title: None,
            priority: None,
            timestamp: None,
            sound: None,
            device: None,
            html: None,
            monospace: None,
            retry: None,
            expire: None,
            callback: None,
        }
    }

    pub fn title(mut self, title: String) -> NewMessage {
        self.title = Some(title);
        self
    }

    pub fn url(mut self, url: String) -> NewMessage {
        self.url = Some(url);
        self
    }

    pub fn url_title(mut self, url_title: String) -> NewMessage {
        self.url_title = Some(url_title);
        self
    }

    pub fn priority(mut self, priority: Priority) -> NewMessage {
        self.priority = Some(priority);
        self
    }

    pub fn timestamp(mut self, timestamp: i64) -> NewMessage {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn sound(mut self, sound: Sound) -> NewMessage {
        self.sound = Some(sound);
        self
    }

    pub fn device(mut self, device: String) -> NewMessage {
        self.device = Some(device);
        self
    }

    pub fn html(mut self, html: bool) -> NewMessage {
        self.html = Some(html);
        self
    }

    pub fn monospace(mut self, monospace: bool) -> NewMessage {
        self.monospace = Some(monospace);
        self
    }

    pub fn retry(mut self, retry: i64) -> NewMessage {
        self.retry = Some(retry);
        self
    }

    pub fn expire(mut self, expire: i64) -> NewMessage {
        self.expire = Some(expire);
        self
    }

    pub fn callback(mut self, callback: String) -> NewMessage {
        self.callback = Some(callback);
        self
    }

    pub fn ok(&self) -> Result<i32, String> {
        let mut errors: Vec<String> = Vec::new();

        if self.message.len() > 1024 {
            errors.push(String::from("Message is too long"));
        } else if self.message.len() == 0 {
            errors.push(String::from("Message is empty"));
        }

        if self.title.is_some() && self.title.as_ref().unwrap().len() > 250 {
            errors.push(String::from("Title is too long"));
        }

        if self.url.is_some() && self.url.as_ref().unwrap().len() > 512 {
            errors.push(String::from("URL is too long"));
        }

        if self.url_title.is_some() && self.url_title.as_ref().unwrap().len() > 100 {
            errors.push(String::from("URL title is too long"));
        }

        if self.device.is_some() && self.device.as_ref().unwrap().len() > 25 {
            errors.push(String::from("Device is too long"));
        }

        if self.callback.is_some() && self.callback.as_ref().unwrap().len() > 512 {
            errors.push(String::from("Callback is too long"));
        }

        // html and monospace are mutually exclusive
        if (self.html.is_some() && *self.html.as_ref().unwrap()) && (self.monospace.is_some() && *self.monospace.as_ref().unwrap()) {
            errors.push(String::from("html and monospace are mutually exclusive"));
        }

        if errors.len() > 0 {
            return Err(errors.join(", "));
        }

        Ok(200)
    }
}
