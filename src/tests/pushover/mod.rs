
// test
#[cfg(test)]
mod tests {
    use tokio_test::block_on;
    use crate::lib::pushover::message::NewMessage;

    use crate::lib::pushover::priority::Priority;
    use crate::lib::pushover::send::send_message;
    use crate::lib::pushover::sounds::Sound;

    macro_rules! aw {
    ($e:expr) => {
        block_on($e)
    };
  }
    #[test]
    fn test_send() {
        let message = NewMessage::new(
            String::from("token"),
            String::from("user"), String::from("message")).title(String::from("title")).priority(Priority::Low).sound(Sound::Pushover).url(String::from("https://google.com")).url_title(String::from("Google")).timestamp(chrono::Utc::now().timestamp()).device(String::from("device")).html(false).monospace(true).retry(60).expire(3600);

        let response = send_message(message);

        assert!(aw!(response).is_ok());
    }
}