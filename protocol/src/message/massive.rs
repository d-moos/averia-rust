use crate::message::message::Message;

struct Massive {
    size: u16,
    remaining: u16,

    messages: Vec<Message>,
    header: Message
}

enum MassiveError {
    HeaderAlreadyPresent,
    MessagesOverflow
}

impl Massive {
    fn try_add(_message: Message) -> Result<(), MassiveError> {
        Ok(())
    }
}