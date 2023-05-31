use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // path to file with eye patterns
    #[arg(short, long)]
    path: String,
}

fn main() {
    // get the command line args
    let args = Args::parse();

    // get an image result
    let img_result = image::open(args.path.clone());

    // if result Ok: convert data to rgba8 ImageBuffer
    let img_buff = match img_result {
        Ok(data) => data.to_rgba8(),
        Err(error) => panic!("Problem opening path: {}. \nError: {:?}", args.path, error)
    };

    // number of pixels the message is offset from the left and top edge of the picture
    let mut message_x_offset = 0;
    let mut message_y_offset = 0;
    // whether or not we have found the message offsets for the picture
    let mut message_offsets_found = false;

    let mut pupil_locations = vec![];

    // this will contain the final processed message. it will be a multi-dimensional vector that
    // has a first dimension length of the message height in trigrams (the messages from the game 
    // have a range from 4 to 6 rows of trigrams) , and a second dimensional length of the
    // message width in trigrams (the messages from the game always have 26 columns of trigrams).
    let mut trigrams: Vec<Vec<Vec<String>>> = vec![];

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

    // go through all our pupil locations, at this point we should know what the message offsets
    // are so we can translate our picture specific coords into message specific coords
    for pupil in pupil_locations.iter() {
        // pupil coords relative to the top left of the message space. sub 3 from the 
        // pupil_message_x as the far left 3 pixels of each message is part of trigram x 
        // index -1 and needs to be ignored for proper tiling
        let pupil_message_x = pupil.0 - message_x_offset - 3;
        let pupil_message_y = pupil.1 - message_y_offset;

        // trigrams have a tileable size of 18 by 14 pixels, we divide our message specific coords
        // by these numbers to get trigram indices that are left unrounded.
        let trigram_x_unrounded = pupil_message_x as f64 / 18f64;
        let trigram_y_unrounded = pupil_message_y as f64 / 14f64;

        // use unrounded trigram indices to get ratios of how far into the trigram's width and
        // height the pupil is
        let pupil_x_ratio = trigram_x_unrounded % 1f64;
        let pupil_y_ratio = trigram_y_unrounded % 1f64;

        // round the trigram coords to get finalized coords and convert to usize so we can use
        // these numbers as rust indices, we're just gonna assume here that you won't have a
        // message with more than your computer architecture's max bit width of trigrams...
        let trigram_x = trigram_x_unrounded.floor() as usize;
        let trigram_y = trigram_y_unrounded.floor() as usize;

        // if there is no message row vector for the row we are on, make one
        if trigrams.get(trigram_y).is_none() {
            trigrams.push(vec![]);
        }

        // if there is no trigram info in the row and column slot we are on, make an empty vec
        // where we will put our eye info
        if trigrams[trigram_y].get(trigram_x).is_none() {
            println!("current row length: {}", trigrams[trigram_y].len());
            println!("trigram_x: {}", trigram_x);
            // TODO: fill empty space between list length and current trigram_x as we don't knwo
            // which trigram_x will be hit and they almost never will be hit in order from 0 to 25
            trigrams[trigram_y].push(vec![]);
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
        println!("pupil_x_ratio: {:?}, pupil_y_ratio: {:?}", pupil_x_ratio, pupil_y_ratio);

        //temp
        trigrams[trigram_y][trigram_x].push(direction.to_string());

        // print that shit
        //println!("{},{} {},{} {}", trigram_x, trigram_y, pixel.0 + 1, pixel.1 + 1, direction);
        
    }

    //println!("image ColorType: {:?}", img.color());
}
