use crate::file_utils::write_to_file;
use glob::glob;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "samples"]
pub struct Asset;

impl Asset {
    /// Takes in the file_path for the sample and returns
    /// the content of the sample as a string
    pub fn handle_sample(file_path: &str) -> String {
        let tail_wind_sample = Asset::get(file_path).unwrap();

        String::from_utf8_lossy(&tail_wind_sample.data).to_string()
    }

    /// Takes in a target file to find and a sample path
    /// if the target file is found the contents from the
    /// sample path is written to the target
    pub fn locate_target_and_write_assets(target: &str, sample_path: &str) {
        let entries = match glob(target) {
            Ok(paths) => paths,
            Err(_) => {
                eprintln!("Something went wrong, please try again later");
                std::process::exit(1)
            }
        };

        for entry in entries {
            match entry {
                Ok(path) => {
                    let content = Self::handle_sample(sample_path);

                    write_to_file(path, content)
                }
                Err(_) => {
                    eprintln!("Something went wrong, please try again later");
                    std::process::exit(1)
                }
            }
        }
    }
}
