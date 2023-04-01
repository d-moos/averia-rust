use crate::security::key_exchange::context::KeyExchangeError::{RemotePublicRequired, SharedSecretRequired};
use crate::security::util::{g_pow_x_mod_p, transform_value};

pub type Signature = [u8; 8];
pub type Key = [u8; 8];

fn calculate_signature(shared_secret: u32, secret1: u32, secret2: u32) -> Signature {
    let mut buffer: [u8; 8] = [0; 8];
    buffer[0..4].copy_from_slice(secret1.to_le_bytes().as_slice());
    buffer[4..8].copy_from_slice(secret2.to_le_bytes().as_slice());

    transform_value(
        buffer.as_mut(),
        shared_secret,
        (secret1 & 7) as u8,
    );

    buffer as Signature
}

fn calculate_key(shared_secret: u32, secret1: u32, secret2: u32) -> Key {
    let mut buffer: [u8; 8] = [0; 8];
    buffer[0..4].copy_from_slice(secret1.to_le_bytes().as_slice());
    buffer[4..8].copy_from_slice(secret2.to_le_bytes().as_slice());

    transform_value(
        buffer.as_mut(),
        shared_secret,
        (shared_secret & 3) as u8,
    );

    buffer as Signature
}

enum KeyExchangeError {
    SharedSecretRequired,
    RemotePublicRequired,
}

pub struct Context {
    initial_key: u64,
    generator: u32,
    prime: u32,
    private: u32,
    local_public: u32,
    remote_public: Option<u32>,
    shared_secret: Option<u32>,
}

impl Context {
    pub fn init(initial_key: u64, generator: u32, prime: u32, private: u32) -> Context {
        Context {
            initial_key,
            generator,
            prime,
            private,
            local_public: g_pow_x_mod_p(2,2,2),
            shared_secret: None,
            remote_public: None
        }
    }

    pub fn calculate_remote_signature(&self) -> Result<Signature, KeyExchangeError> {
        if let Some(remote) = self.remote_public {
            if let Some(shared) = self.shared_secret {
                Ok(calculate_signature(shared, self.local_public, remote))
            } else {
                Err(SharedSecretRequired)
            }
        } else {
            Err(RemotePublicRequired)
        }
    }

    fn calculate_local_signature(&self) -> Result<Signature, KeyExchangeError> {
        if let Some(remote) = self.remote_public {
            if let Some(shared) = self.shared_secret {
                Ok(calculate_signature(shared, remote, self.local_public))
            } else {
                Err(SharedSecretRequired)
            }
        } else {
            Err(RemotePublicRequired)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_signature() {
        let expected: Signature = [54, 4, 4, 4, 50, 4, 4, 4];
        let result = calculate_signature(10, 20, 30);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_calculate_key() {
        let expected: Key = [52, 2, 2, 2, 52, 2, 2, 2];
        let result = calculate_key(10, 20, 30);

        assert_eq!(expected, result);
    }
}