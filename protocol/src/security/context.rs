use crate::security::error_checking::checksum::Checksum;
use crate::security::error_checking::sequencer::Sequencer;

struct EncodingContext {
    requires_security_bytes: bool,
    sequencer: Sequencer,
    checksum: Checksum,

    initial_key: u64,
    generator: u32,
    prime: u32,
    private: u32,
    local_public: u32,
    remote_public: u32,
    shared_secret: u32
}