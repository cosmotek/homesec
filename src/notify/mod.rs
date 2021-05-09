extern crate chrono;

use twilio::{Client, OutboundMessage};
use chrono::prelude::*;
use joinery::Joinable;

pub struct Notifier {
    client: Client,
    from_number: String,
    to_numbers: Vec<String>,
}

impl Notifier {
    pub async fn notify_entry(n: &Self, entry_ids: Vec<String>) {
        let now = Local::now();
        let (is_pm, hour) = now.hour12();

        let display_time = format!(
            "{:?} {:02}-{:02}-{:02} @ {:02}:{:02}:{:02} {}",
            now.weekday(),
            now.month(),
            now.day(),
            now.year(),
            hour,
            now.minute(),
            now.second(),
            if is_pm { "PM" } else { "AM" },
        );

        let msg_body = format!(
            "HomeSec detected entry by{}:\n\n{}\n\nSent: {}",
            if entry_ids.len() == 1 { "" } else { " these entities"},
            entry_ids.join_with("\n").to_string(),
            display_time,
        );

        println!("Sending message: \n'{}'\n", msg_body);
        let msg = OutboundMessage::new(&n.from_number, &n.to_numbers[0], &msg_body);

        match n.client.send_message(msg).await {
            Ok(m) => println!("{:?}", m),
            Err(e) => eprintln!("{:?}", e),
        }
    }

    // fn notify_exit(n: &self, exit_ids: Vec<String>) {

    // }
}

pub fn new(twilio_app_id: String, twilio_auth_token: String, from_number: String, to_numbers: Vec<String>) -> Notifier {
    Notifier{
        client: Client::new(&twilio_app_id, &twilio_auth_token),
        from_number: String::from(from_number),
        to_numbers: to_numbers,
    }
}