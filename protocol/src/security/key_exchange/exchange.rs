use crate::security::key_exchange::context::{Context, Signature};

const RANDOM_U64: u64 = 123;
const RANDOM_U32: u32 = 123;

struct SenderStrategy(Context);

impl SenderStrategy {
    fn init() -> SenderStrategy {
        SenderStrategy {
            0: Context::init(RANDOM_U64, RANDOM_U32, RANDOM_U32, RANDOM_U32)
        }
    }

    fn ack(&mut self, local_public: u32, signature: Signature) {
    }
}


struct ReceiverStrategy(Context);

impl ReceiverStrategy {
}