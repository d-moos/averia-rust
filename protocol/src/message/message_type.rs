const NONE: u8 = 0;
const NET_ENGINE: u8 = 1;
const FRAMEWORK: u8 = 2;
const GAME: u8 = 3;

#[derive(Clone)]
pub enum MessageType {
    None,
    NetEngine,
    Framework,
    Game
}

impl Into<u8> for MessageType {
    fn into(self) -> u8 {
        match self {
            MessageType::None => NONE,
            MessageType::NetEngine => NET_ENGINE,
            MessageType::Framework => FRAMEWORK,
            MessageType::Game => GAME
        }
    }
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            NONE => MessageType::None,
            NET_ENGINE => MessageType::NetEngine,
            FRAMEWORK => MessageType::Framework,
            GAME => MessageType::Game,
            _ => panic!("unknown value")
        }
    }
}