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

    // we will search the image for pixels that form the iris/pupil of the eyes
    // the iris/pupil in this case is a plus shape of black pixels
    // we will then check various pixels in relation to the plus center pixel
    // to see which eye direction we have located
    let iris = vec![vec![true, false, true], vec![false, false, false], vec![true, false, true]];

    // for every pixel in the image
    for pixel in img_buff.enumerate_pixels() {
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

                // if the state ever doesn't match, break out of the iris_loop as no pupil has been
                // found
                if template_state ^ pixel_state {
                    pupil_located = false;
                    break 'iris_loop;
                }
            }
        }

        // we found an iris!
        if pupil_located {
            // calc pupil coords in picture
            let pupil_x = pixel.0 + 1;
            let pupil_y = pixel.1 + 1;

            // calc trigram indices
            let trigram_x = ((pupil_x - 3) as f64 / 18f64).floor();
            let trigram_y = (pupil_y as f64 / 14f64).floor();

            // check for pupil direction
            let mut direction = "c";
            if img_buff.get_pixel(pupil_x, pupil_y + 2)[0] <= 0 {
                direction = "d"
            } else if img_buff.get_pixel(pupil_x, pupil_y - 2)[0] <= 0 {
                direction = "u"
            } else if img_buff.get_pixel(pupil_x + 1, pupil_y - 2)[0] <= 0 {
                direction = "r"
            } else if img_buff.get_pixel(pupil_x - 1, pupil_y - 2)[0] <= 0 {
                direction = "l"
            }

            // print that shit
            println!("{},{} {},{} {}", trigram_x, trigram_y, pixel.0 + 1, pixel.1 + 1, direction);
        }
    }

    //println!("image ColorType: {:?}", img.color());
}
