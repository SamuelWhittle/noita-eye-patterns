use crate::Trigrams;

pub mod unique_triangles;

pub fn decode_arg_match(decode_type: String, trigrams: Trigrams) {
    match decode_type.as_str() {
        "unique_triangles" => {
            unique_triangles::test();
        }
        _ => {
            println!("unknown trigram decode method specified.")
        }
    }
}
