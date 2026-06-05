use konto_crypto::aead;
use konto_crypto::keystore::{KeyMode, Keystore};
use konto_crypto::password::{self, KdfParams};
use konto_crypto::Dek;

#[test]
fn aead_roundtrip_and_tamper_detection() {
    let key = [3u8; 32];
    let sealed = aead::seal(&key, b"hello world").unwrap();
    assert_ne!(&sealed, b"hello world");
    assert_eq!(aead::open(&key, &sealed).unwrap(), b"hello world");

    // Wrong key fails.
    assert!(aead::open(&[4u8; 32], &sealed).is_err());

    // Tampered ciphertext fails (GCM auth).
    let mut bad = sealed.clone();
    *bad.last_mut().unwrap() ^= 0xff;
    assert!(aead::open(&key, &bad).is_err());

    // Two seals of the same plaintext differ (random nonce).
    let again = aead::seal(&key, b"hello world").unwrap();
    assert_ne!(sealed, again);
}

#[test]
fn dek_hex_and_material_parsing() {
    let dek = Dek::from_bytes([0xABu8; 32]);
    let hex = dek.to_hex();
    assert_eq!(hex.len(), 64);
    let parsed = Dek::from_str_material(&hex).unwrap();
    assert_eq!(parsed.to_hex(), hex);

    // Bad length rejected.
    assert!(Dek::from_str_material("deadbeef").is_err());
}

#[test]
fn password_wrap_unwrap_roundtrip() {
    // Small params keep the test fast.
    let kdf = fast_kdf();
    let dek = Dek::from_bytes([7u8; 32]);

    let wrapped = password::wrap_dek(&dek, "correct horse", &kdf).unwrap();
    let unwrapped = password::unwrap_dek(&wrapped, "correct horse", &kdf).unwrap();
    assert_eq!(unwrapped.to_hex(), dek.to_hex());

    // Wrong password fails as InvalidPassword.
    match password::unwrap_dek(&wrapped, "wrong", &kdf) {
        Err(konto_crypto::CryptoError::InvalidPassword) => {}
        Ok(_) => panic!("wrong password should not unwrap"),
        Err(e) => panic!("unexpected error: {e}"),
    }
}

#[test]
fn keystore_password_roundtrip_serialization() {
    let kdf = fast_kdf();
    let dek = Dek::from_bytes([5u8; 32]);
    let wrapped = password::wrap_dek(&dek, "pw", &kdf).unwrap();

    let ks = Keystore::password(kdf, &wrapped);
    assert_eq!(ks.mode, KeyMode::Password);

    let dir = std::env::temp_dir().join(format!("konto-ks-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    ks.save(&dir).unwrap();

    let loaded = Keystore::load(&dir).unwrap().unwrap();
    assert_eq!(loaded.mode, KeyMode::Password);
    let recovered = password::unwrap_dek(
        &loaded.wrapped_dek_bytes().unwrap(),
        "pw",
        loaded.kdf.as_ref().unwrap(),
    )
    .unwrap();
    assert_eq!(recovered.to_hex(), dek.to_hex());

    // Keychain-mode keystore carries no secret material.
    Keystore::keychain().save(&dir).unwrap();
    let kc = Keystore::load(&dir).unwrap().unwrap();
    assert_eq!(kc.mode, KeyMode::Keychain);
    assert!(kc.wrapped_dek.is_none());

    std::fs::remove_dir_all(&dir).ok();
}

/// Argon2 params with minimal cost so tests stay fast.
fn fast_kdf() -> KdfParams {
    let mut kdf = KdfParams::new_random().unwrap();
    kdf.mem_kib = 8;
    kdf.iters = 1;
    kdf.parallelism = 1;
    kdf
}
