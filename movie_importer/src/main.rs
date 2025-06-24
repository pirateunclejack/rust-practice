use movie_importer::read_txt_to_json;
use std::{error::Error, process};

use rfd::FileDialog;

fn main() -> Result<(), Box<dyn Error>> {
    match FileDialog::new()
        .add_filter("Text Files", &["txt"])
        .set_title("Select the DVD text file")
        .set_directory(r"/home/jack/workspace/rust/movie_importer")
        .pick_file()
    {
        Some(file_path) => {
            let saved_path = read_txt_to_json(&file_path)?;
            println!("Data saved to: {saved_path:?}");
            Ok(())
        }
        None => {
            eprintln!("File not selected.");
            process::exit(-1)
        }
    }
}
