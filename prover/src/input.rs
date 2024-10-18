use std::{collections::HashMap, str::FromStr};

use ethabi::{decode, encode, ethereum_types::U256, ParamType, Token};
use num_bigint::{BigInt, Sign};
use serde::{Deserialize, Serialize};
use ark_circom::zkp::Input;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CryptoRumbleInput {
    #[serde(rename = "fromSeed")]
    pub from_seed: String,
    #[serde(rename = "toSeed")]
    pub to_seed: String,
    #[serde(rename = "fromBoard")]
    pub from_board: Vec<Vec<u8>>,
    #[serde(rename = "toBoard")]
    pub to_board: Vec<Vec<u8>>,
    pub step: u64,
    #[serde(rename = "stepAfter")]
    pub step_after: u64,
    #[serde(rename = "fromBoardPacked")]
    pub from_board_packed: String,
    #[serde(rename = "toBoardPacked")]
    pub to_board_packed: String,
    #[serde(rename = "scorePacked")]
    pub score_packed: String,
    #[serde(rename = "posPacked")]
    pub pos_packed: String,
    #[serde(rename = "itemPacked")]
    pub item_packed: String,
    pub moves: Vec<Vec<u8>>,
    pub arg: Vec<u8>,
}

const BOARD_STEP: u32 = 32;
const BOARD_LEN: usize = 6;
const SCORE_STEP: u32 = 2048;
const SCORE_LEN: usize = 5;
const MOVE_LEN: usize = 30;

fn unpack(t: Token, step: u32, len: usize) -> Vec<BigInt> {
    let mut d = t.into_uint().unwrap_or(U256::zero());
    let step = U256::from(step);
    let mut items = vec![];

    loop {
        if d < step {
            items.push(BigInt::from(d.as_u64()));
            break;
        }
        let (next, n) = d.div_mod(step);

        d = next;
        items.push(BigInt::from(n.as_u64()));
    }

    if items.len() < len {
        for _ in items.len()..len {
            items.push(BigInt::from(0));
        }
    }

    items.reverse();
    return items;
}

// pos = pos * 64 + m[0] * 8 + m[1];
// item = item * 256 + m[2];
fn unpack_move(pos: Token, item: Token) -> Vec<BigInt> {
    let m3_items = unpack(item, 256, MOVE_LEN);
    let mut p = pos.into_uint().unwrap_or(U256::zero());
    let step1 = U256::from(64);
    let step2 = U256::from(8);

    let mut m1_items = vec![];
    let mut m2_items = vec![];
    loop {
        if p < step1 {
            let (m1, m2) = p.div_mod(step2);
            m1_items.push(BigInt::from(m1.as_u64()));
            m2_items.push(BigInt::from(m2.as_u64()));
            break;
        }

        let (next, n) = p.div_mod(step1);
        p = next;

        // n = 8 * x + y
        let (m1, m2) = n.div_mod(step2);
        m1_items.push(BigInt::from(m1.as_u64()));
        m2_items.push(BigInt::from(m2.as_u64()));
    }

    if m1_items.len() < MOVE_LEN {
        for _ in m1_items.len()..MOVE_LEN {
            m1_items.push(BigInt::from(0));
            m2_items.push(BigInt::from(0));
        }
    }

    m1_items.reverse();
    m2_items.reverse();

    let mut items = vec![];
    for i in 0..MOVE_LEN {
        items.push(m1_items[i].clone());
        items.push(m2_items[i].clone());
        items.push(m3_items[i].clone());
    }

    items
}

#[allow(dead_code)]
pub fn encode_prove_inputs_publics(input: &CryptoRumbleInput) -> (String, String) {
    let mut inputs_items = vec![];
    let mut publics_items = vec![];

    let score_packed = BigInt::from_str(&input.score_packed)
        .unwrap()
        .to_bytes_be()
        .1;
    let score_packed = Token::Uint(U256::from_big_endian(score_packed.as_slice()));
    inputs_items.push(score_packed.clone());

    let scores = unpack(score_packed, SCORE_STEP, SCORE_LEN);
    for score in scores {
        let score_bytes = score.to_bytes_be().1;
        publics_items.push(Token::Uint(U256::from_big_endian(score_bytes.as_slice())));
    }

    let from_seed = BigInt::from_str(&input.from_seed).unwrap().to_bytes_be().1;
    let from_seed = Token::Uint(U256::from_big_endian(from_seed.as_slice()));
    inputs_items.push(from_seed.clone());
    publics_items.push(from_seed);

    let to_seed = BigInt::from_str(&input.to_seed).unwrap().to_bytes_be().1;
    let to_seed = Token::Uint(U256::from_big_endian(to_seed.as_slice()));
    inputs_items.push(to_seed.clone());
    publics_items.push(to_seed);

    let step = Token::Uint(U256::from_big_endian(input.step.to_be_bytes().as_slice()));
    inputs_items.push(step.clone());
    publics_items.push(step);

    let step_after = Token::Uint(U256::from_big_endian(
        input.step_after.to_be_bytes().as_slice(),
    ));
    inputs_items.push(step_after.clone());
    publics_items.push(step_after);

    let from_board_packed = BigInt::from_str(&input.from_board_packed)
        .unwrap()
        .to_bytes_be()
        .1;
    let from_board_packed = Token::Uint(U256::from_big_endian(from_board_packed.as_slice()));
    inputs_items.push(from_board_packed.clone());
    publics_items.push(from_board_packed);

    let to_board_packed = BigInt::from_str(&input.to_board_packed)
        .unwrap()
        .to_bytes_be()
        .1;
    let to_board_packed = Token::Uint(U256::from_big_endian(to_board_packed.as_slice()));
    inputs_items.push(to_board_packed.clone());
    publics_items.push(to_board_packed);

    let pos_packed = BigInt::from_str(&input.pos_packed).unwrap().to_bytes_be().1;
    let pos_packed = Token::Uint(U256::from_big_endian(pos_packed.as_slice()));
    inputs_items.push(pos_packed.clone());
    publics_items.push(pos_packed);

    let item_packed = BigInt::from_str(&input.item_packed).unwrap().to_bytes_be().1;
    let item_packed = Token::Uint(U256::from_big_endian(item_packed.as_slice()));
    inputs_items.push(item_packed.clone());
    publics_items.push(item_packed);

    let inputs_bytes = encode(&inputs_items);
    let publics_bytes = encode(&publics_items);

    (format!("0x{}", hex::encode(&inputs_bytes)), format!("0x{}", hex::encode(&publics_bytes))) 
}

pub fn decode_prove_inputs(bytes: &[u8]) -> Result<Input, anyhow::Error> {
    let input_tokens = decode(
        &[
            ParamType::Uint(256), // scorePacked
            ParamType::Uint(256), // fromSeed
            ParamType::Uint(256), // toSeed
            ParamType::Uint(256), // step
            ParamType::Uint(256), // stepAfter
            ParamType::Uint(256), // fromBoardPacked
            ParamType::Uint(256), // toBoardPacked
            ParamType::Uint(256), // posPacked
            ParamType::Uint(256), // itemPacked
        ],
        bytes,
    )?;

    let f_uint = |token: Token| -> BigInt {
        let mut bytes = [0u8; 32];
        token.into_uint().unwrap().to_big_endian(&mut bytes);
        BigInt::from_bytes_be(Sign::Plus, &bytes)
    };

    let score_packed = f_uint(input_tokens[0].clone());

    let from_seed = f_uint(input_tokens[1].clone());
    let to_seed = f_uint(input_tokens[2].clone());
    let step = f_uint(input_tokens[3].clone());
    let step_after = f_uint(input_tokens[4].clone());

    let from_board_packed = f_uint(input_tokens[5].clone());
    let to_board_packed = f_uint(input_tokens[6].clone());
    let from_board = unpack(input_tokens[5].clone(), BOARD_STEP, BOARD_LEN);
    let to_board = unpack(input_tokens[6].clone(), BOARD_STEP, BOARD_LEN);

    let pos_packed = f_uint(input_tokens[7].clone());
    let item_packed = f_uint(input_tokens[8].clone());
    let moves = unpack_move(input_tokens[7].clone(), input_tokens[8].clone());

    let arg = vec![BigInt::from(0); 30]; // arg use default [0; 30];

    let mut maps = HashMap::new();
    maps.insert("fromSeed".to_string(), vec![from_seed]);
    maps.insert("toSeed".to_string(), vec![to_seed]);
    maps.insert("fromBoard".to_string(), from_board);
    maps.insert("toBoard".to_string(), to_board);
    maps.insert("step".to_string(), vec![step]);
    maps.insert("stepAfter".to_string(), vec![step_after]);
    maps.insert("fromBoardPacked".to_string(), vec![from_board_packed]);
    maps.insert("toBoardPacked".to_string(), vec![to_board_packed]);
    maps.insert("scorePacked".to_string(), vec![score_packed]);
    maps.insert("posPacked".to_string(), vec![pos_packed]);
    maps.insert("itemPacked".to_string(), vec![item_packed]);
    maps.insert("move".to_string(), moves);
    maps.insert("arg".to_string(), arg);

    Ok(Input { maps })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize() {
        let input = r##"
        {
            "fromSeed": "16938986816621673014406792984620325385232245869428348395053494538472250137768",
            "toSeed": "18809534718515133310982073931212903285152506282303066166330452480033125747936",
            "fromBoard": [
                [2, 5, 2, 3, 3, 4],
                [4, 4, 5, 1, 4, 3],
                [1, 2, 4, 5, 2, 3],
                [1, 4, 2, 3, 5, 1],
                [2, 3, 2, 1, 5, 3],
                [1, 3, 3, 2, 2, 5]
            ],
            "toBoard": [
                [5, 2, 2, 3, 4, 5],
                [4, 2, 2, 5, 1, 1],
                [2, 3, 5, 2, 3, 4],
                [5, 1, 2, 2, 4, 3],
                [2, 5, 3, 3, 2, 1],
                [3, 2, 2, 1, 4, 2]
            ],
            "step": 0,
            "stepAfter": 19,
            "fromBoardPacked": "103361923205923181585452685177869704657870687575312453",
            "toBoardPacked": "242543694228480640306188505874996485086797052824847490",
            "scorePacked": "387165653630999",
            "posPacked": "407069173718415000365340272682837370791232631116922880",
            "itemPacked": "13803492696795028375627839078134363494882806125467409972850288729522176",
            "moves": [
                [2, 1, 2],
                [0, 0, 0],
                [0, 0, 0],
                [0, 0, 0],
                [4, 0, 2],
                [0, 0, 0],
                [1, 3, 1],
                [3, 4, 2],
                [2, 4, 2],
                [1, 5, 1],
                [0, 0, 0],
                [0, 0, 0],
                [0, 2, 1],
                [2, 2, 2],
                [0, 1, 1],
                [0, 0, 0],
                [0, 3, 1],
                [0, 1, 1],
                [0, 3, 1],
                [0, 3, 2],
                [0, 3, 1],
                [0, 1, 1],
                [0, 2, 1],
                [2, 2, 2],
                [0, 0, 0],
                [0, 2, 1],
                [0, 5, 1],
                [0, 0, 0],
                [0, 0, 0],
                [0, 0, 0]
            ],
            "arg": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        }
        "##;

        let input: CryptoRumbleInput = serde_json::from_str(input).unwrap();
        let (input_hex, publics_hex) = encode_prove_inputs_publics(&input);
        std::fs::write("test_input", &input_hex).expect("Unable to create test_input file");
        std::fs::write("test_output", &publics_hex).expect("Unable to create test_output file");

        let input_hex = input_hex.trim_start_matches("0x");
        let input_bytes = hex::decode(input_hex).expect("Unable to decode input file");
        decode_prove_inputs(&input_bytes).expect("Unable to decode input");
    }
}
