use std::io::{Cursor, Read};
use std::mem::size_of;
use bytes::{Buf, Bytes};
use tokio::io::AsyncReadExt;
use protocol::message::message::Message;
use crate::consumer::consume::Consume;

struct ErrorDetectionSeed {
    sequence: u32,
    checksum: u32,
}

impl From<[u8; 8]> for ErrorDetectionSeed {
    fn from(value: [u8; 8]) -> Self {
        let mut bytes = Bytes::from(value.to_vec());
        ErrorDetectionSeed {
            sequence: bytes.get_u32(),
            checksum: bytes.get_u32()
        }
    }
}

struct Setup {
    key: u64,
    generator: u32,
    prime: u32,
    public: u32,
}

impl From<Bytes> for Setup {
    fn from(mut value: Bytes) -> Self {
        Setup {
            key: value.get_u64(),
            generator: value.get_u32(),
            prime: value.get_u32(),
            public: value.get_u32()
        }
    }
}

type Signature = [u8;8];

struct Request {
    option: u8,
    insecure_key: Option<u64>,
    error_detection_seed: Option<ErrorDetectionSeed>,
    setup: Option<Setup>,
    challenge: Option<Signature>,
}

const ENC_NONE: u8 = 0;
const ENC_ENCRYPTION: u8 = 1 << 1;
const ENC_ERR_DETECTION: u8 = 1 << 2;
const ENC_KEY_EXCHANGE: u8 = 1 << 3;
const ENC_KEY_CHALLENGE: u8 = 1 << 4;


impl From<&[u8]> for Request {
    fn from(value: &[u8]) -> Self {
        let mut bytes = Bytes::from(value.to_vec());
        let mut request = Request {
            option: bytes.get_u8(),
            setup: None,
            challenge: None,
            error_detection_seed: None,
            insecure_key: None
        };

        if request.option == ENC_NONE {
            return request;
        }

        if request.option & ENC_ENCRYPTION != 0 {
            request.insecure_key = Some(bytes.get_u64());
        }

        if request.option & ENC_ERR_DETECTION != 0 {
            let mut buffer = [0u8;8];
            bytes.reader().read_exact(&mut buffer).unwrap();
            request.error_detection_seed = Some(ErrorDetectionSeed::from(buffer));

            let s = size_of::<ErrorDetectionSeed>();
        }

        if request.option & ENC_KEY_EXCHANGE != 0 {
            request.setup = Some(Setup::from(bytes));
        }

        request
    }
}

impl From<Bytes> for Request {
    fn from(mut value: Bytes) -> Self {
        let mut request = Request {
            option: value.get_u8(),
            setup: None,
            challenge: None,
            error_detection_seed: None,
            insecure_key: None
        };

        if request.option == ENC_NONE {
            return request;
        }

        if request.option & ENC_ENCRYPTION != 0 {
            request.insecure_key = Some(value.get_u64());
        }

        if request.option & ENC_ERR_DETECTION != 0 {
            request.error_detection_seed = Some(ErrorDetectionSeed::from(value));
        }

        if request.option & ENC_KEY_EXCHANGE != 0 {
            request.setup = Some(Setup::from(value));
        }


        request
    }
}


impl Consume for Request {
    fn consume(&self, message: Message) {
        todo!()
    }
}