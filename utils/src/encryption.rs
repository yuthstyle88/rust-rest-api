// extern crate rand;

use std::str;

use binascii::{bin2hex, ConvertError, hex2bin};
use crypto::{aes, blockmodes, buffer, symmetriccipher};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use rand::prelude::*;
use rand::rngs::OsRng;

// const BUFFER_SIZE: usize = 4096;
// Encrypt a buffer with the given key and iv using
// AES-256/CBC/Pkcs encryption.
pub fn encrypt(data: String, key: &[u8]) -> Result<String, symmetriccipher::SymmetricCipherError> {
    let mut iv: [u8; 16] = [0u8; 16];
    let mut rng = OsRng::default();
    rng.fill_bytes(&mut iv);

    // Create an encryptor instance of the best performing
    // type available for the platform.
    let mut encryptor = aes::cbc_encryptor(
        aes::KeySize::KeySize256,
        key,
        &iv,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data.as_bytes());
    let mut buffer = [0u8; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true);
        match result {
            Ok(buffer_result) => {
                final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

                match buffer_result {
                    BufferResult::BufferUnderflow => break,
                    BufferResult::BufferOverflow => {}
                }
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

    let iv_buffer = &mut [0u8; 32];
    let iv_hex = bin2hex(&iv, iv_buffer).ok().unwrap();

    let data_buffer = &mut [0u8; 4096];
    match bin2hex(&final_result, data_buffer) {
        Ok(data_hex) => {
            let iv_str = str::from_utf8(iv_hex).unwrap();
            let data_str = str::from_utf8(data_hex).unwrap();

            let encrypted_data = format!("{}:{}", iv_str, data_str);

            Ok(encrypted_data)
        }
        Err(err) => {
            match err {
                ConvertError::InvalidInputLength => { println!("Invalid Input Length"); }
                ConvertError::InvalidOutputLength => { println!("Invalid Output Length"); }
                ConvertError::InvalidInput => { println!("Invalid Input"); }
            }

            Err(symmetriccipher::SymmetricCipherError::InvalidLength)
        }
    }
}

// Decrypts a buffer with the given key and iv using
// AES-256/CBC/Pkcs encryption.
pub fn decrypt(data: String, key: &[u8]) -> Result<String, symmetriccipher::SymmetricCipherError> {
    let encrypted: Vec<&str> = data.split(":").collect();

    let vi_buffer = &mut [0u8; 32];
    let iv = hex2bin(encrypted[0].as_ref(), vi_buffer).ok().unwrap();
    // println!("IV: {:?}", &iv);

    let data_buffer = &mut [0u8; 4096];
    let enc_data = hex2bin(encrypted[1].as_ref(), data_buffer).ok().unwrap();
    // println!("ENC DATA: {:?}", &enc_data);

    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(enc_data);
    let mut buffer = [0u8; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    let decrypted = str::from_utf8(&final_result).unwrap();

    Ok(decrypted.to_string())
}


#[cfg(test)]
mod tests {
    use crate::constants;
    use crate::utils::encryption::{decrypt, encrypt};

    #[test]
    fn test_encrypt_decrypt() {
        let source = "aa1234".to_string();
        let encrypt_text = encrypt(source.clone(), &constants::KEY).unwrap();
        dbg!(&encrypt_text);
        let decrypt_text = decrypt(encrypt_text, &constants::KEY).unwrap();
        dbg!(&decrypt_text);
        assert_eq!(source, decrypt_text);
    }

    #[test]
    fn test_encrypt() {
        let encrypt_text = encrypt("aa1234".to_string(), &constants::KEY).unwrap();
        dbg!(&encrypt_text);
        assert_eq!(65, encrypt_text.len());
    }

    #[test]
    fn test_decrypt() {
        let decrypt_text = decrypt("162db6b0568a9762317b9a36aa2f16c0:5fcd0682d063b6f9c3d269c90e432085".to_string(), &constants::KEY).unwrap();
        dbg!(&decrypt_text);
        assert_eq!("$AAbb12345".to_string(), decrypt_text);
    }
}