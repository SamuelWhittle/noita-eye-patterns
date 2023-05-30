use clap::Parser;
use image::GenericImageView;

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
    let imgBuff = match img_result {
        Ok(data) => data.to_rgba8(),
        Err(error) => panic!("Problem opening path: {}. \nError: {:?}", args.path, error)
    };

    // we will search the image for pixels that form the iris/pupil of the eyes
    // the iris/pupil in this case is a plus shape of black pixels
    // we will then check various pixels in relation to the plus center pixel
    // to see which eye direction we have located
    let iris = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
    iris.iter().enumerate().for_each(|(x, column)| {
        column.iter().enumerate().for_each(|(y, state)| {
            println!("({}, {}), state: {}", x, y, state);
        })
        //println!("next: {:?}", iter.next());
    })

    //for pixel in imgBuff.enumerate_pixels() {
    //    println!("location: {:?}, {:?}", pixel.0, pixel.1);

    //}

    //println!("image ColorType: {:?}", img.color());
}
