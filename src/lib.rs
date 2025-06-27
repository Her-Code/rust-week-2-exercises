use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    // TODO: Decode hex string into Vec<u8>, return error string on failure
    match hex::decode(hex_str) {
        Ok(bytes) => Ok(bytes), 
        Err(e) => Err(e.to_string()), 
    }
    
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    // TODO: Reverse the byte order of input slice and return as Vec<u8>
    let mut result = Vec::new(); 

    // Iterate over the bytes in reverse order
    for i in (0..bytes.len()).rev() {
        result.push(bytes[i]); 
    }

    result // Return the reversed vector
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    // TODO: Implement conversion of bytes slice to hex string
    let mut hex_string = String::new(); 

    // Iterate over each byte
    for &byte in bytes {
        
        hex_string.push_str(&format!("{:02x}", byte));
    }

    hex_string 
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    // TODO: Implement conversion of hex string to bytes vector
    hex::decode(hex)
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    // TODO: Implement little-endian byte swap for u32
    [
        (num & 0xFF) as u8,           
        ((num >> 8) & 0xFF) as u8,     
        ((num >> 16) & 0xFF) as u8,    
        ((num >> 24) & 0xFF) as u8,    
    ]
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    // TODO: Parse input string to u64, return error string if invalid
    let trimmed = input.trim();

    // Check if the input is empty after trimming
    if trimmed.is_empty() {
        return Err("Input is empty. Please enter a number.".to_string());
    }

    // Try to convert the trimmed string into a u64 number
    match trimmed.parse::<u64>() {
        Ok(value) => Ok(value), 
        Err(_) => Err("Invalid satoshi amount".to_string()),
    }
}

pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}


pub fn classify_script(script: &[u8]) -> ScriptType {
    // TODO: Match script pattern and return corresponding ScriptType
    // For P2PKH: check if we have at least 3 bytes with OP_DUP, OP_HASH160, and push length 0x14
    if script.len() >= 3 &&
       script[0] == 0x76 && 
       script[1] == 0xa9 && 
       script[2] == 0x14    
    {
        return ScriptType::P2PKH;
    }

    // Adjusted P2WPKH check: Now match on the first two bytes being 0x00 and 0x14
    if script.len() >= 3 && script[0] == 0x00 && script[1] == 0x14 {
        return ScriptType::P2WPKH;
    }

    // If none match, return Unknown
    ScriptType::Unknown
}


// TODO: complete Outpoint tuple struct
pub struct Outpoint(pub String, pub u32);

pub fn read_pushdata(script: &[u8]) -> &[u8] {
    // TODO: Return the pushdata portion of the script slice (assumes pushdata starts at index 2)
    if script.len() <= 2 {
        &[]
    } else {
        &script[2..]
    }
}

pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        // TODO: Return the wallet's confirmed balance
        self.confirmed
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    // TODO: Subtract fee from mutable balance reference
    if *balance >= fee {
        *balance -= fee;
    } else {
        *balance = 0;
    }
}

pub fn move_txid(txid: String) -> String {
    // TODO: Return formatted string including the txid for display or logging
    format!("txid: {}", txid)
}

// TODO: Add necessary derive traits
#[derive(PartialEq, Debug)]
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        // TODO: Implement mapping from byte to Opcode variant
        match byte {
            0xAC => Ok(Opcode::OpChecksig), 
            0x76 => Ok(Opcode::OpDup),    
            _ => Err(format!("Invalid opcode: 0x{:02X}", byte)), 
        }
    }
}

// TODO: Add necessary derive traits
#[derive(Clone, PartialEq, Debug)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    // TODO: Implement UTXO consumption logic (if any)
    // Placeholder: In a real blockchain, this might mark the UTXO as spent
    utxo
}
