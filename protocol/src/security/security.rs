use crate::security::error_checking::checksum::Checksum;
use crate::security::error_checking::sequencer::Sequencer;

// temp
struct Blowfish;

struct Security {
    blowfish: Blowfish,
    sequencer: Sequencer,
    checksum: Checksum
}

impl Security {
    fn encode() {}

    fn decode() {}
}