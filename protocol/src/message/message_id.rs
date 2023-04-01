use crate::message::message_type::{MessageType};

const OPERATION_SIZE: u16 = 12;
const OPERATION_OFFSET: u16 = 0;
const OPERATION_MASK: u16 = ((1 << OPERATION_SIZE) - 1) << OPERATION_OFFSET;

const TYPE_SIZE: u16 = 2;
const TYPE_OFFSET: u16 = OPERATION_OFFSET + OPERATION_SIZE;
const TYPE_MASK: u16 = ((1 << TYPE_SIZE) - 1) << TYPE_OFFSET;

const DIRECTION_SIZE: u16 = 2;
const DIRECTION_OFFSET: u16 = TYPE_OFFSET + TYPE_SIZE;
const DIRECTION_MASK: u16 = ((1 << DIRECTION_SIZE) - 1) << DIRECTION_OFFSET;


pub struct MessageId {
    direction: u8,
    message_type: MessageType,
    operation: u16,
}

impl Clone for MessageId {
    fn clone(&self) -> Self {
        MessageId {
            direction: self.direction.clone(),
            operation: self.operation.clone(),
            message_type: self.message_type.clone(),
        }
    }
}

impl From<u16> for MessageId {
    fn from(value: u16) -> Self {
        MessageId {
            direction: ((value & DIRECTION_MASK) >> DIRECTION_OFFSET) as u8,
            message_type: MessageType::from(((value & TYPE_MASK) >> TYPE_OFFSET) as u8),
            operation: ((value & OPERATION_MASK) >> OPERATION_OFFSET),
        }
    }
}

impl Into<u16> for MessageId {
    fn into(self) -> u16 {
        let mut value: u16 = 0;
        value = ((value & !DIRECTION_MASK) | (((self.direction as u16) << DIRECTION_OFFSET) as u16 & DIRECTION_MASK)) as u16;
        value = ((value & !TYPE_MASK) | (((Into::<u8>::into(self.message_type) as u16) << TYPE_OFFSET) as u16 & TYPE_MASK)) as u16;
        value = ((value & !OPERATION_MASK) | (((self.operation as u16) << OPERATION_OFFSET) as u16 & OPERATION_MASK)) as u16;
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::message::message_id::MessageId;

    #[test]
    fn back_and_forth_works() {
        let opcode: u16 = 0xA101;
        let id = MessageId::from(opcode);

        let new_opcode: u16 = id.into();
        assert_eq!(new_opcode, opcode);
    }
}