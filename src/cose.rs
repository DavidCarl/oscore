use crate::cbor::encode;
use crate::error::Error;
use alloc::vec::Vec;
use ed25519_dalek::{Keypair, Signature};
use serde::{Deserialize, Serialize};
use sha2::Sha512;

#[derive(Debug, Serialize, Deserialize)]
struct SigStructure<'a>(
    &'a str,
    #[serde(with = "serde_bytes")] &'a [u8],
    #[serde(with = "serde_bytes")] &'a [u8],
    #[serde(with = "serde_bytes")] &'a [u8],
);

/// Returns the signature from signing the `Sig_structure` of the given data.
///
/// # Arguments
/// * `id_cred_x` - The CBOR encoded header map identifying a public
///   authentication key, e.g. `{ 4 : h'1111' }`
/// * `th_i` - The transcript hash
/// * `cred_x` - Encoded `COSE_Key`
/// * `keypair_bytes` - The ed25519 authentication key pair. First 32 bytes are
///   the secret key, the other 32 bytes the public key.
pub fn sign(
    id_cred_x: &[u8],
    th_i: &[u8],
    cred_x: &[u8],
    keypair_bytes: &[u8],
) -> Result<[u8; 64], Error> {
    let to_be_signed = build_to_be_signed(id_cred_x, th_i, cred_x)?;
    let keypair = Keypair::from_bytes(&keypair_bytes)?;
    let signature = keypair.sign::<Sha512>(&to_be_signed);

    Ok(signature.to_bytes())
}

/// Checks if the signature was made on a `Sig_structure` of the given data,
/// with the given key.
///
/// # Arguments
/// * `id_cred_x` - The CBOR encoded header map identifying a public
///   authentication key, e.g. `{ 4 : h'1111' }`
/// * `th_i` - The transcript hash
/// * `cred_x` - Encoded `COSE_Key`
/// * `public_key` - The ed25519 public key of the pair used for the signature
/// * `signature` - The ed25519 signature
pub fn verify(
    id_cred_x: &[u8],
    th_i: &[u8],
    cred_x: &[u8],
    public_key: &[u8],
    signature: &[u8],
) -> Result<(), Error> {
    let to_be_signed = build_to_be_signed(id_cred_x, th_i, cred_x)?;
    let public_key = ed25519_dalek::PublicKey::from_bytes(public_key)?;
    let signature = Signature::from_bytes(signature)?;

    Ok(public_key.verify::<Sha512>(&to_be_signed, &signature)?)
}

fn build_to_be_signed(
    id_cred_x: &[u8],
    th_i: &[u8],
    cred_x: &[u8],
) -> Result<Vec<u8>, Error> {
    // Create the Sig_structure
    let sig_struct = SigStructure("Signature1", id_cred_x, th_i, cred_x);

    encode(&sig_struct)
}

#[cfg(test)]
mod tests {
    use super::*;

    static ID_CRED_X: [u8; 5] = [0xA1, 0x04, 0x42, 0x11, 0x11];
    static TH_I: [u8; 3] = [0x22, 0x22, 0x22];
    static CRED_X: [u8; 4] = [0x55, 0x55, 0x55, 0x55];
    static M: [u8; 27] = [
        0x84, 0x6a, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65,
        0x31, 0x45, 0xA1, 0x04, 0x42, 0x11, 0x11, 0x43, 0x22, 0x22, 0x22,
        0x44, 0x55, 0x55, 0x55, 0x55,
    ];

    static SIGNATURE: [u8; 64] = [
        0x51, 0xA9, 0xD7, 0xCA, 0x97, 0x8E, 0x09, 0x41, 0x5A, 0xC3, 0x76,
        0x28, 0x46, 0x27, 0x12, 0xAC, 0x9D, 0xA9, 0xBD, 0xF3, 0x68, 0x2F,
        0xC4, 0x47, 0xB3, 0x06, 0x5E, 0x1B, 0x1E, 0x92, 0xAA, 0x4C, 0x3B,
        0x03, 0x95, 0x02, 0x9D, 0x6C, 0xF9, 0xF7, 0xF6, 0x73, 0x4F, 0x7C,
        0xEC, 0xE0, 0x3B, 0xAB, 0x71, 0xDB, 0x90, 0x2B, 0xC3, 0x9D, 0xA5,
        0x1B, 0x8D, 0xB7, 0x34, 0xCD, 0xD9, 0x87, 0x99, 0x06,
    ];
    static KEYPAIR: [u8; 64] = [
        0xF4, 0x20, 0x6A, 0x9E, 0xFA, 0x0A, 0xF5, 0xEF, 0x1F, 0x66, 0x88,
        0xBC, 0xAF, 0xDA, 0xF8, 0x16, 0x0C, 0xC5, 0x88, 0x54, 0x5C, 0x24,
        0x08, 0xF1, 0x8C, 0xAF, 0x8C, 0x8F, 0xA6, 0xE7, 0x67, 0x75, 0xAA,
        0x71, 0xD1, 0xFE, 0xB3, 0xD7, 0xD7, 0x8C, 0x14, 0x7F, 0xBD, 0xCA,
        0xAD, 0x34, 0x67, 0x88, 0xC2, 0x44, 0x32, 0x3E, 0xC6, 0x4D, 0x9A,
        0x85, 0x68, 0x6D, 0x4D, 0x06, 0xA9, 0x58, 0x6F, 0x20,
    ];

    #[test]
    fn to_be_signed() {
        let to_be_signed =
            build_to_be_signed(&ID_CRED_X, &TH_I, &CRED_X).unwrap();
        assert_eq!(&to_be_signed, &M);
    }

    #[test]
    fn signature_same() {
        let signature = sign(&ID_CRED_X, &TH_I, &CRED_X, &KEYPAIR).unwrap();
        assert_eq!(&SIGNATURE[..], &signature[..]);
    }

    #[test]
    fn signature_verifies() {
        let signature = sign(&ID_CRED_X, &TH_I, &CRED_X, &KEYPAIR).unwrap();
        assert!(verify(
            &ID_CRED_X,
            &TH_I,
            &CRED_X,
            &KEYPAIR[32..],
            &signature
        )
        .is_ok());

        let mut cred_x_changed = CRED_X.to_vec();
        cred_x_changed[1] = 0x44;
        assert!(verify(
            &ID_CRED_X,
            &TH_I,
            &cred_x_changed,
            &KEYPAIR[32..],
            &signature
        )
        .is_err());
    }

}
