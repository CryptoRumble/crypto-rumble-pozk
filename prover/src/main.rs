mod input;

use ark_circom::zkp::{
    init_bn254_circom_from_bytes, init_bn254_params_from_bytes, proof_to_abi_bytes,
    prove_bn254, verify_bn254, decode_prove_publics
};
use input::{decode_prove_inputs};

const WASM_BYTES: &[u8] = include_bytes!("../materials/crypto_rumble_30.wasm");
const R1CS_BYTES: &[u8] = include_bytes!("../materials/crypto_rumble_30.r1cs");
const ZKEY_BYTES: &[u8] = include_bytes!("../materials/crypto_rumble_30.zkey");

/// INPUT=http://localhost:9098/tasks/1 cargo run --release
#[tokio::main]
async fn main() {
    let input_path = std::env::var("INPUT").expect("env INPUT missing");
    let bytes = reqwest::get(&input_path)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    // parse inputs and publics
    let mut input_len_bytes = [0u8; 4];
    input_len_bytes.copy_from_slice(&bytes[0..4]);
    let input_len = u32::from_be_bytes(input_len_bytes) as usize;
    let input_bytes = &bytes[4..input_len + 4];
    let publics_bytes = &bytes[input_len + 4..];

    let input = decode_prove_inputs(input_bytes).expect("Unable to decode inputs");
    let publics = decode_prove_publics(publics_bytes, 13).expect("Unable to decode publics");

    let params = init_bn254_params_from_bytes(ZKEY_BYTES, false).unwrap();
    let circom = init_bn254_circom_from_bytes(WASM_BYTES, R1CS_BYTES).unwrap();

    let (_pi, proof) = prove_bn254(&params, circom, input).unwrap();
    assert!(verify_bn254(&params.vk, &publics, &proof).unwrap());

    let bytes = proof_to_abi_bytes(&proof).unwrap();
    let client = reqwest::Client::new();
    client.post(&input_path).body(bytes).send().await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_crypto_rumble() {
        // inputs & publics are same
        let bytes = hex::decode("000000000000000000000000000000000000000000000000000160200741001725732074ddc361ac18a1b5eb47ee1bae165a5db3804f31fac2bfce36f2cfdca82995d1c2b1e1a46b369ccb1497dbf0924f7c09dfeef92f4b35d0d73e1712cce00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001300000000000000000001144319084284830888510c2410ca110c4128c2318845000000000000000000028843214821142110ca2190a11088311463104621048200000000000000000004400008002dc50d0000920400c10c30c10920021400000000020000000200010202010000010201000101010201010102000101000000").unwrap();
        let bytes2 = hex::decode("00000000000000000000000000000000000000000000000000000000000000160000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001d0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000001725732074ddc361ac18a1b5eb47ee1bae165a5db3804f31fac2bfce36f2cfdca82995d1c2b1e1a46b369ccb1497dbf0924f7c09dfeef92f4b35d0d73e1712cce00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001300000000000000000001144319084284830888510c2410ca110c4128c2318845000000000000000000028843214821142110ca2190a11088311463104621048200000000000000000004400008002dc50d0000920400c10c30c10920021400000000020000000200010202010000010201000101010201010102000101000000").unwrap();

        let input = decode_prove_inputs(&bytes).expect("Unable to decode inputs");
        let publics = decode_prove_publics(&bytes2, 13).expect("Unable to decode publics");

        let params = init_bn254_params_from_bytes(ZKEY_BYTES, false).unwrap();
        let circom = init_bn254_circom_from_bytes(WASM_BYTES, R1CS_BYTES).unwrap();
        let (pi, proof) = prove_bn254(&params, circom, input).unwrap();
        assert_eq!(pi, publics);

        let bytes = proof_to_abi_bytes(&proof).unwrap();
        println!("0x{}", hex::encode(bytes));

        assert!(verify_bn254(&params.vk, &publics, &proof).unwrap());
    }
}
