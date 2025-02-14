use quiche::ConnectionId;
use ring::hmac;
use ring::rand;
use std::io::Write;
use std::io::{
    self,
};
use std::net::IpAddr;
use std::net::SocketAddr;

use crate::QuicResultExt;

const HMAC_TAG_LEN: usize = 32;

pub(crate) struct AddrValidationTokenManager {
    sign_key: hmac::Key,
}

impl Default for AddrValidationTokenManager {
    fn default() -> Self {
        let rng = rand::SystemRandom::new();

        AddrValidationTokenManager {
            sign_key: hmac::Key::generate(hmac::HMAC_SHA256, &rng).unwrap(),
        }
    }
}

impl AddrValidationTokenManager {
    pub(super) fn gen(
        &self, original_dcid: &[u8], client_addr: SocketAddr,
    ) -> Vec<u8> {
        let ip_bytes = match client_addr.ip() {
            IpAddr::V4(addr) => addr.octets().to_vec(),
            IpAddr::V6(addr) => addr.octets().to_vec(),
        };

        let token_len = HMAC_TAG_LEN + ip_bytes.len() + original_dcid.len();
        let mut token = io::Cursor::new(vec![0u8; token_len]);

        token.set_position(HMAC_TAG_LEN as u64);
        token.write_all(&ip_bytes).unwrap();
        token.write_all(original_dcid).unwrap();

        let tag = hmac::sign(&self.sign_key, &token.get_ref()[HMAC_TAG_LEN..]);

        token.set_position(0);
        token.write_all(tag.as_ref()).unwrap();

        token.into_inner()
    }

    pub(super) fn validate_and_extract_original_dcid<'t>(
        &self, token: &'t [u8], client_addr: SocketAddr,
    ) -> io::Result<ConnectionId<'t>> {
        let ip_bytes = match client_addr.ip() {
            IpAddr::V4(addr) => addr.octets().to_vec(),
            IpAddr::V6(addr) => addr.octets().to_vec(),
        };

        let hmac_and_ip_len = HMAC_TAG_LEN + ip_bytes.len();

        if token.len() < hmac_and_ip_len {
            return Err("token is too short").into_io();
        }

        let (tag, payload) = token.split_at(HMAC_TAG_LEN);

        hmac::verify(&self.sign_key, payload, tag)
            .map_err(|_| "signature verification failed")
            .into_io()?;

        if payload[..ip_bytes.len()] != *ip_bytes {
            return Err("IPs don't match").into_io();
        }

        Ok(ConnectionId::from_ref(&token[hmac_and_ip_len..]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate() {
        let manager = AddrValidationTokenManager::default();

        let assert_tag_generated = |token: &[u8]| {
            let tag = &token[..HMAC_TAG_LEN];
            let all_nulls = tag.iter().all(|b| *b == 0u8);

            assert!(!all_nulls);
        };

        let token = manager.gen(b"foo", "127.0.0.1:1337".parse().unwrap());

        assert_tag_generated(&token);
        assert_eq!(token[HMAC_TAG_LEN..HMAC_TAG_LEN + 4], [127, 0, 0, 1]);
        assert_eq!(&token[HMAC_TAG_LEN + 4..], b"foo");

        let token = manager.gen(b"bar", "[::1]:1338".parse().unwrap());

        assert_tag_generated(&token);

        assert_eq!(token[HMAC_TAG_LEN..HMAC_TAG_LEN + 16], [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
        ]);

        assert_eq!(&token[HMAC_TAG_LEN + 16..], b"bar");
    }

    #[test]
    fn validate() {
        let manager = AddrValidationTokenManager::default();

        let addr = "127.0.0.1:1337".parse().unwrap();
        let token = manager.gen(b"foo", addr);

        assert_eq!(
            manager
                .validate_and_extract_original_dcid(&token, addr)
                .unwrap(),
            ConnectionId::from_ref(b"foo")
        );

        let addr = "[::1]:1338".parse().unwrap();
        let token = manager.gen(b"barbaz", addr);

        assert_eq!(
            manager
                .validate_and_extract_original_dcid(&token, addr)
                .unwrap(),
            ConnectionId::from_ref(b"barbaz")
        );
    }

    #[test]
    fn validate_err_short_token() {
        let manager = AddrValidationTokenManager::default();
        let v4_addr = "127.0.0.1:1337".parse().unwrap();
        let v6_addr = "[::1]:1338".parse().unwrap();

        for addr in &[v4_addr, v6_addr] {
            assert!(manager
                .validate_and_extract_original_dcid(b"", *addr)
                .is_err());

            assert!(manager
                .validate_and_extract_original_dcid(&[1u8; HMAC_TAG_LEN], *addr)
                .is_err());

            assert!(manager
                .validate_and_extract_original_dcid(
                    &[1u8; HMAC_TAG_LEN + 1],
                    *addr
                )
                .is_err());
        }
    }

    #[test]
    fn validate_err_ips_mismatch() {
        let manager = AddrValidationTokenManager::default();

        let token = manager.gen(b"foo", "127.0.0.1:1337".parse().unwrap());

        assert!(manager
            .validate_and_extract_original_dcid(
                &token,
                "127.0.0.2:1337".parse().unwrap()
            )
            .is_err());

        let token = manager.gen(b"barbaz", "[::1]:1338".parse().unwrap());

        assert!(manager
            .validate_and_extract_original_dcid(
                &token,
                "[::2]:1338".parse().unwrap()
            )
            .is_err());
    }

    #[test]
    fn validate_err_invalid_signature() {
        let manager = AddrValidationTokenManager::default();

        let addr = "127.0.0.1:1337".parse().unwrap();
        let mut token = manager.gen(b"foo", addr);

        token[..HMAC_TAG_LEN].copy_from_slice(&[1u8; HMAC_TAG_LEN]);

        assert!(manager
            .validate_and_extract_original_dcid(&token, addr)
            .is_err());
    }
}
