use image::io::Reader as ImageReader;
use std::error::Error;
use std::fs;

fn main() {
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        let path = path.unwrap();
        if !path.file_type().unwrap().is_file() {
            continue;
        }
        let path = path.path().display().to_string();

        if let Err(err) = invert(&path) {
            println!("{}: {}", path, err);
        }
    }
}

fn invert(name: &str) -> Result<(), Box<dyn Error>> {
    let mut img = ImageReader::open(name)?.decode()?;
    img.invert();
    img.save(name)?;
    Ok(())
}
