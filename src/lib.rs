use image::GenericImageView;
use std::process;

pub struct Arguments {
    pub picture1: String,
    pub picture2: String,
}

impl Arguments {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Arguments, &'static str> {
        args.next();
        let picture1 = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't receive neither pictures"),
        };
        let picture2 = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't receive second picture"),
        };
        Ok(Arguments { picture1, picture2 })
    }
}

fn image_equality(pictures: &Arguments) -> bool {
    let img1 = image::open(pictures.picture1.clone()).unwrap_or_else(|err| {
        eprintln!("Picture opening picture 1: {}", err);
        process::exit(1);
    });
    let img2 = image::open(pictures.picture2.clone()).unwrap_or_else(|err| {
        eprintln!("Picture opening picture 2: {}", err);
        process::exit(1);
    });
    if img1.height() != img2.height() || img1.width() != img2.width() {
        return false;
    };
    for width in 0..(img1.width() - 1) {
        for height in 0..(img1.height() - 1) {
            let img1_pixel = img1.get_pixel(width, height);
            let img2_pixel = img2.get_pixel(width, height);
            if img1_pixel.ne(&img2_pixel) {
                return false;
            };
        }
    }
    return true;
}

pub fn run(pics: &Arguments) {
    if image_equality(pics) {
        println!("The images are equal, Hooray");
    } else {
        println!("The images are not the same, boo!");
    }
}
