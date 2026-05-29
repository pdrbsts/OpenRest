// Verifica que a nossa cifra AES-128-ECB/PKCS7 é compatível com a do PHP/OpenSSL.
// Encripta 'testes1234' com uma chave fixa e imprime o resultado base64. Deve
// coincidir com:
//   echo -n testes1234 | openssl enc -aes-128-ecb -K $(echo -n 0123456789abcdef | xxd -p -c100) -nopad
// Mais correctamente, com padding PKCS7:
//   echo -n testes1234 | openssl enc -aes-128-ecb -K 30313233343536373839616263646566 -base64

use aes::Aes128;
use base64::{engine::general_purpose, Engine as _};
use cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyInit};
use ecb::Encryptor;

type Aes128EcbEnc = Encryptor<Aes128>;

fn aes_ecb_pkcs7(key: &[u8], data: &[u8]) -> Vec<u8> {
    let cipher = Aes128EcbEnc::new_from_slice(key).unwrap();
    let mut buf = vec![0u8; data.len() + 16];
    buf[..data.len()].copy_from_slice(data);
    let n = cipher.encrypt_padded_mut::<Pkcs7>(&mut buf, data.len()).unwrap().len();
    buf.truncate(n);
    buf
}

fn main() {
    let key = b"0123456789abcdef"; // 16 bytes ASCII
    let data = b"testes1234";
    let ciphertext = aes_ecb_pkcs7(key, data);
    println!("key  (hex): {}", key.iter().map(|b| format!("{:02x}", b)).collect::<String>());
    println!("data : {:?}", String::from_utf8_lossy(data));
    println!("cipher (b64): {}", general_purpose::STANDARD.encode(&ciphertext));
    println!("cipher (hex): {}", ciphertext.iter().map(|b| format!("{:02x}", b)).collect::<String>());
}
