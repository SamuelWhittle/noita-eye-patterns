use std::collections::HashMap;
use image::{RgbaImage};
use std::num::ParseIntError;

pub mod unique_triangles;

pub type PupilLocation = (u32, u32);
pub type TrigramMessage = Vec<Vec<String>>;

// take a trigram state such as "clr" and return its associated usize value
pub fn trigram_state_to_decimal(trigram_state: String) -> usize {
    // map between an eye state and a corresponding number with a radix of 5
    let iris_state_map: HashMap<char, char> = HashMap::from([('c', '0'), ('l', '1'), ('r', '2'), ('u', '3'), ('d', '4')]);

    // try to get a usize by going through every character in the string and swapping it with our
    // associated radix 5 number e.g.("clr" -> "012"), then using from_str_radix to return 
    let radix_10_index: Result<usize, ParseIntError> = usize::from_str_radix(trigram_state.chars().map(|dir| {
        iris_state_map.get(&dir).unwrap().clone()
    }).collect::<String>().as_str(), 5);

    match radix_10_index {
        Ok(num) => num,
        Err(err) => panic!("{:?}", err)
    }
}

pub fn print_trigram_msg<T: std::fmt::Display>(trigram_msg: Vec<Vec<T>>) {
    print!("trigram_msg: \n");
    for row in trigram_msg {
        for item in row {
            print!("{:>2}, ", item.to_string())
        }

        print!("\n");
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

// ##### image processing #####

pub fn get_img_buff(path: &String) -> RgbaImage {
    // get an image result
    let img_result = image::open(path);

    // if result Ok: convert data to rgba8 ImageBuffer
    let img_buff = match img_result {
        Ok(data) => data.to_rgba8(),
        Err(error) => panic!("Problem opening path: {}. \nError: {:?}", path, error)
    };
    
    img_buff
}

pub fn process_pixels(img_buff: &RgbaImage) -> Vec<PupilLocation> {
    let mut message_x_offset = 0;
    let mut message_y_offset = 0;
    // whether or not we have found the message offsets for the picture
    let mut message_offsets_found = false;

    // list of all pupil (x, y) coords in the image
    let mut pupil_locations: Vec<PupilLocation> = vec![];

    // we will search the image for pixels that form the iris/pupil of the eyes
    // the iris/pupil in this case is a plus shape of black pixels
    // we will then check various pixels in relation to the plus center pixel
    // to see which eye direction we have located
    let iris = vec![vec![true, false, true], vec![false, false, false], vec![true, false, true]];
    let left_corner_template = vec![
        vec![true, true, true, false, true, true, true], 
        vec![true, true, false, true, false, true, true], 
        vec![true, false, true, true, true, false, true]
    ];

    // for every pixel in the image
    for pixel in img_buff.enumerate_pixels() {
        // if we have not found the first left eye corner, we don't yet know the exact message
        // offset within the image, so check if the current area matches the left eye corner
        // template. once we have found the message offsets this will be skipped
        if !message_offsets_found {
            'left_corner_loop: for x in 0..=2 {
                for y in 0..=6 {
                    // check to see if we are too close to the picture edges for an eye
                    if pixel.0 > img_buff.dimensions().0 - 11 || pixel.1 > img_buff.dimensions().1 - 7 {
                        break 'left_corner_loop;
                    }

                    let template_state = left_corner_template[x][y];
                    let pixel_state = img_buff.get_pixel(pixel.0 + u32::try_from(x).unwrap(), pixel.1 + u32::try_from(y).unwrap())[0] > 0;

                    if template_state ^ pixel_state {
                        break 'left_corner_loop;
                    }

                    if x == 2 && y == 6 {
                        message_offsets_found = true;
                        message_x_offset = pixel.0;
                        message_y_offset = pixel.1;
                    }
                }
            }
        }

        //println!("location: ({:?}, {:?}), color: {:?}", pixel.0, pixel.1, pixel.2);
        if pixel.0 > img_buff.dimensions().0 - 3 || pixel.1 > img_buff.dimensions().1 - 3 {
            continue;
        }

        let mut pupil_located = true;

        // iterate over every pixel in the template
        'iris_loop: for x in 0..=2 {
            for y in 0..=2 {
                let template_state = iris[x][y];
                let pixel_state = img_buff.get_pixel(pixel.0 + u32::try_from(x).unwrap(), pixel.1 + u32::try_from(y).unwrap())[0] > 0;

                // if the state ever doesn't match, break out of the iris_loop as no iris has been
                // found
                if template_state ^ pixel_state {
                    pupil_located = false;
                    break 'iris_loop;
                }
            }
        }

        // if we didn't locate an iris, continue in the loop with no further adieu
        if !pupil_located {
            continue;
        }

        // we found an iris!

        // calc pupil coords in picture
        let pupil_x = pixel.0 + 1;
        let pupil_y = pixel.1 + 1;
        // push our pupil coords to the pupil vector
        pupil_locations.push((pupil_x, pupil_y));
    }

    pupil_locations.iter().map(|coords| {
        (coords.0 - message_x_offset, coords.1 - message_y_offset)
    }).collect()
}

pub fn process_pupils(pupil_locations: Vec<PupilLocation>, img_buff: RgbaImage) -> TrigramMessage {
    let mut trigrams: Vec<Vec<Vec<String>>> = vec![];

    // go through all our pupil locations, at this point we should know what the message offsets
    // are so we can translate our picture specific coords into message specific coords
    for pupil in pupil_locations.iter() {
        // pupil coords relative to the top left of the message space. sub 3 from the 
        // pupil_message_x as the far left 3 pixels of each message is part of trigram x 
        // index -1 and needs to be ignored for proper tiling
        let pupil_message_x = pupil.0 - 3;
        let pupil_message_y = pupil.1;

        // trigrams have a tileable size of 18 by 14 pixels, we divide our pixel X coord
        // by 18 to get an unrounded trigram X coord.
        let trigram_x_unrounded = pupil_message_x as f64 / 18f64;

        // use unrounded trigram X coord to get ratios of how far into the trigram's width
        // the pupil is
        let pupil_x_ratio = trigram_x_unrounded % 1f64;

        // round the trigram X coord and calculate the trigram Y coord and convert to usize so we can use
        // these numbers as rust indices, these indices are where the trigram in question is
        // located within the tiled message. We're just gonna assume here that you won't have a
        // message with more than your computer architecture's max bit width of trigrams...
        let trigram_x = trigram_x_unrounded.floor() as usize;
        let trigram_y = (pupil_message_y as f64 / 14f64).floor() as usize;

        // if there is no message row vector for the row we are on, make one
        if trigrams.get(trigram_y).is_none() {
            trigrams.push(vec![]);
        }

        // if there is no index in which our trigram should be, push empty vectors into the row
        // until the column we need has a place to put the trigram
        if trigrams[trigram_y].get(trigram_x).is_none() {
            for _ in trigrams[trigram_y].len()..=trigram_x {
                trigrams[trigram_y].push(vec!["".to_string(); 3]);
            }
        }

        // check pixels surrounding the iris to see which eye state we are looking at
        let mut direction = "c";
        if img_buff.get_pixel(pupil.0, pupil.1 + 2)[0] <= 0 {
            direction = "d";
        } else if img_buff.get_pixel(pupil.0, pupil.1 - 2)[0] <= 0 {
            direction = "u";
        } else if img_buff.get_pixel(pupil.0 + 1, pupil.1 - 2)[0] <= 0 {
            direction = "r";
        } else if img_buff.get_pixel(pupil.0 - 1, pupil.1 - 2)[0] <= 0 {
            direction = "l";
        }

        // figure out which eye in the trigram we are looking at
        // trigrams are indexed from left to right like so:
        // 0 2| 1 |0 2
        //  1 |0 2| 1 
        let mut pupil_index: usize = 1;
        if pupil_x_ratio < 0.37 {
            pupil_index = 0;
        } else if pupil_x_ratio > 0.51 {
            pupil_index = 2;
        }

        trigrams[trigram_y][trigram_x][pupil_index] = direction.to_string();
    }

    trigrams.iter().map(|message_row| {
        message_row.clone().iter().map(|trigram| {
            trigram.clone().iter().fold("".to_string(), |acc, eye_state| {
                acc + eye_state
            })
        }).collect()
    }).collect()
}
