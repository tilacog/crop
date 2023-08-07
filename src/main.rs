use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::{env, io};

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <input_image_path> <rows> <columns>", args[0]);
        std::process::exit(1);
    }

    // Open the input image
    let input_path = &args[1];
    let mut img = image::open(input_path)?;

    // Get the dimensions of the input image
    let (width, height) = (img.width(), img.height());

    // Rows and columns
    let rows:u32 = args[2].parse()?;
    let columns: u32 = args[3].parse()?;

    // Define the size of the sub-images
    let sub_image_width = width / columns;
    let sub_image_height = height / rows;

    // Create an output directory with the same basename as the input image
    let output_dir = create_output_dir(input_path)?;

    // Slice the input image into sub-images and save each sub-image
    for i in 0..columns {
        for j in 0..rows {
            let sub_image_x = i * sub_image_width;
            let sub_image_y = j * sub_image_height;

            let sub_image = img.crop(sub_image_x, sub_image_y, sub_image_width, sub_image_height);

            let output_path = format!("subimage_{}_{}.jpg", i + 1, j + 1);
            sub_image.save(output_dir.join(output_path))?
        }
    }

    println!(
        "Sliced {} into {} sub-images of size {}x{}",
        input_path,
        rows * columns,
        sub_image_width,
        sub_image_height
    );
    Ok(())
}

fn create_output_dir(input_path: &str) -> io::Result<PathBuf> {
    let input_filename = Path::new(input_path).file_stem().unwrap();
    let output_dir = Path::new("sub_images").join(input_filename);
    fs::create_dir_all(&output_dir)?;
    Ok(output_dir)
}
