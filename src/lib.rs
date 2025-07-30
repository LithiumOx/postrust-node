use napi::Result;

use brotli::Decompressor;
use fst::{
    automaton::{Automaton, Str},
    IntoStreamer, Map, Streamer,
};
use napi_derive::napi;
use std::io::Read;
use std::sync::OnceLock;

// Embed the compressed data directly in the binary
static COMPRESSED_DATA: &[u8] = include_bytes!("../postcode_data.br");

// Global state for the loaded data
static POSTCODE_DATA: OnceLock<PostcodeData> = OnceLock::new();

#[derive(Debug)]
struct PostcodeData {
    fst_map: Map<Vec<u8>>,
    house_data: Vec<u8>,
}

/// Result structure for postcode lookups
#[napi(object)]
pub struct LookupResult {
    pub postcode: String,
    pub straat: String,
    pub huisnummer: u32,
    pub woonplaats: String,
}

/// Initialize the postcode data (called automatically on module load)
#[napi]
pub fn init() -> Result<()> {
    let _ = POSTCODE_DATA.get_or_init(|| load_data());
    Ok(())
}

/// Get information about the loaded data
#[napi]
pub fn get_info() -> Result<String> {
    let data = POSTCODE_DATA.get_or_init(|| load_data());
    let memory_usage = (data.fst_map.as_fst().as_bytes().len() + data.house_data.len()) as f64 / 1_000_000.0;
    Ok(format!(
        "postRUST NPM Package\nMemory usage: {:.2} MB\nCompressed data size: {:.2} MB",
        memory_usage,
        COMPRESSED_DATA.len() as f64 / 1_000_000.0
    ))
}

/// Lookup a postcode and house number
#[napi]
pub fn lookup(postcode: String, huisnummer: u32) -> Result<Option<LookupResult>> {
    let data = POSTCODE_DATA.get_or_init(|| load_data());
    Ok(lookup_address_fst(data, &postcode, huisnummer))
}

/// Lookup multiple postcodes at once (batch operation)
#[napi]
pub fn lookup_batch(queries: Vec<(String, u32)>) -> Result<Vec<Option<LookupResult>>> {
    let data = POSTCODE_DATA.get_or_init(|| load_data());
    let results = queries
        .iter()
        .map(|(postcode, huisnummer)| lookup_address_fst(data, postcode, *huisnummer))
        .collect();
    Ok(results)
}

// === Internal implementation (same as in the main server) ===

fn load_data() -> PostcodeData {
    let mut decompressor = Decompressor::new(COMPRESSED_DATA, 4096);
    let mut decompressed_data = Vec::new();
    decompressor
        .read_to_end(&mut decompressed_data)
        .expect("Could not decompress brotli data");

    let fst_len = u64::from_le_bytes(decompressed_data[0..8].try_into().unwrap()) as usize;
    let house_data_len = u64::from_le_bytes(decompressed_data[8..16].try_into().unwrap()) as usize;

    let fst_bytes = decompressed_data[16..16 + fst_len].to_vec();
    let house_data_bytes = decompressed_data[16 + fst_len..16 + fst_len + house_data_len].to_vec();

    let fst_map = Map::new(fst_bytes).expect("FST data is corrupted or invalid");

    PostcodeData {
        fst_map,
        house_data: house_data_bytes,
    }
}

fn lookup_address_fst(
    data: &PostcodeData,
    postcode: &str,
    house_number: u32,
) -> Option<LookupResult> {
    let postcode_upper = postcode.to_uppercase();
    let prefix = format!("{}|", postcode_upper);

    let automaton = Str::new(&prefix).starts_with();
    let mut stream = data.fst_map.search(automaton).into_stream();

    while let Some((key_bytes, offset)) = stream.next() {
        let key_str = std::str::from_utf8(key_bytes).unwrap_or("");
        let house_numbers_compressed = &data.house_data[offset as usize..];
        let house_numbers = decompress_house_numbers(house_numbers_compressed);

        if house_numbers.binary_search(&house_number).is_ok() {
            let parts: Vec<&str> = key_str.split('|').collect();
            if parts.len() == 3 {
                return Some(LookupResult {
                    postcode: postcode_upper,
                    straat: parts[1].to_string(),
                    huisnummer: house_number,
                    woonplaats: parts[2].to_string(),
                });
            }
        }
    }
    None
}

fn decompress_house_numbers(mut compressed_data: &[u8]) -> Vec<u32> {
    if compressed_data.len() < 2 {
        return Vec::new();
    }
    let len = u16::from_le_bytes(compressed_data[0..2].try_into().unwrap()) as usize;
    if len == 0 {
        return Vec::new();
    }
    let mut nums = Vec::with_capacity(len);
    compressed_data = &compressed_data[2..];

    if compressed_data.len() < 2 {
        return nums;
    }

    let first_num = u16::from_le_bytes(compressed_data[0..2].try_into().unwrap()) as u32;
    nums.push(first_num);
    compressed_data = &compressed_data[2..];

    let mut last_num = first_num;
    while nums.len() < len {
        if compressed_data.is_empty() {
            break;
        }
        let delta_marker = compressed_data[0];
        compressed_data = &compressed_data[1..];

        let delta = if delta_marker == 0 {
            if compressed_data.len() < 2 {
                break;
            }
            let d = u16::from_le_bytes(compressed_data[0..2].try_into().unwrap());
            compressed_data = &compressed_data[2..];
            d as u32
        } else {
            delta_marker as u32
        };

        let new_num = last_num + delta;
        nums.push(new_num);
        last_num = new_num;
    }
    nums
}
