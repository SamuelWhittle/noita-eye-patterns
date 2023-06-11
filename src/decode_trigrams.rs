use std::collections::HashMap;
use std::str::Chars;

use crate::TrigramMessage;

pub mod unique_triangles;

pub fn trigram_state_to_decimal(trigram_state: &str) -> usize {
    let iris_state_map: HashMap<char, char> = HashMap::from([('c', '0'), ('l', '1'), ('r', '2'), ('u', '3'), ('d', '4')]);

    match usize::from_str_radix(trigram_state.chars().map(|dir| {
        iris_state_map.get(&dir).unwrap().clone()
    }).collect::<String>().as_str(), 5) {
        Ok(num) => num,
        Err(err) => panic!("{:?}", err)
    }
}

pub fn decode_arg_match(decode_type: String, trigram_msg: TrigramMessage) {
    match decode_type.as_str() {
        "unique_triangles" => {
            unique_triangles::decode(trigram_msg);
        }
        _ => {
            println!("unknown trigram decode method specified.")
        }
    }
}
