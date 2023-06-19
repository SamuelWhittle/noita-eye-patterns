use clap::Parser;
use serde_json::Result;

pub mod decode_trigrams;

use crate::decode_trigrams::TrigramMessage;
use crate::decode_trigrams::get_img_buff;
use crate::decode_trigrams::process_pixels;
use crate::decode_trigrams::process_pupils;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // path to file with eye patterns
    #[arg(short = 'i', long)]
    images: Vec<String>,
    #[arg(short = 'p', long)]
    print_trigrams: bool,
    #[arg(short = 'd', long)]
    decode_type: Option<String>,
}

fn main() -> Result<()> {
    // get the command line args
    let args = Args::parse();

    // does the user want to print the serialized message?
    let print_serialized_trigrams = args.print_trigrams;
    // does the user want to attempt a decode method?
    let mut attempt_decode = false;
    let mut decode_type = "".to_string();

    // check if decode_type flag
    if let Some(flag_data) = args.decode_type {
        decode_type = flag_data;
        attempt_decode = true;
    }

    for path in args.images.clone().iter() {
        //println!("\nrunning on path {}: <{}>", index+1, path);
        let img_buff = get_img_buff(&path);

        // number of pixels the message is offset from the left and top edge of the picture
        let pupil_locations = process_pixels(&img_buff);

        // this will contain the final processed message. it will be a multi-dimensional vector that
        // has a first dimension length of the message height in trigrams (the messages from the game 
        // have a range from 4 to 6 rows of trigrams) , and a second dimensional length of the
        // message width in trigrams (the messages from the game always have 26 columns of trigrams).
        let trigram_msg: TrigramMessage = process_pupils(pupil_locations, img_buff);

        if print_serialized_trigrams {
            // serialize trigrams into json
            let json = serde_json::to_string(&trigram_msg)?;
            println!("{}", json);
        }

        if attempt_decode {
            decode_trigrams::decode_arg_match(decode_type.clone(), trigram_msg);
        }
    }

    Ok(())
}

