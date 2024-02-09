// Module: pushover
#[derive(Debug)]
pub enum Sound {
    Pushover, Bike, Bugle, CashRegister, Classical, Cosmic, Falling, Gamelan, Incoming, Intermission,
    Magic, Mechanical, PianoBar, Siren, SpaceAlarm, Tugboat, Alien, Climb, Persistent, Echo, UpDown,
    Vibrate, None
}

impl Sound {
    pub fn as_str(&self) -> String {
        match self {
            Sound::Pushover => String::from("pushover"),
            Sound::Bike => String::from("bike"),
            Sound::Bugle => String::from("bugle"),
            Sound::CashRegister => String::from("cashregister"),
            Sound::Classical => String::from("classical"),
            Sound::Cosmic => String::from("cosmic"),
            Sound::Falling => String::from("falling"),
            Sound::Gamelan => String::from("gamelan"),
            Sound::Incoming => String::from("incoming"),
            Sound::Intermission => String::from("intermission"),
            Sound::Magic => String::from("magic"),
            Sound::Mechanical => String::from("mechanical"),
            Sound::PianoBar => String::from("pianobar"),
            Sound::Siren => String::from("siren"),
            Sound::SpaceAlarm => String::from("spacealarm"),
            Sound::Tugboat => String::from("tugboat"),
            Sound::Alien => String::from("alien"),
            Sound::Climb => String::from("climb"),
            Sound::Persistent => String::from("persistent"),
            Sound::Echo => String::from("echo"),
            Sound::UpDown => String::from("updown"),
            Sound::Vibrate => String::from("vibrate"),
            Sound::None => String::from("none"),
        }
    }
}