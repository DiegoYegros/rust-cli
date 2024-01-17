use reqwest::blocking::get;
use std::env;
use std::fs::File;
use std::io::Write;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    const URL: &str = "https://cataas.com/cat";
    let response = get(URL)?;
    let status_code = response.status();
    let img_data = response.bytes()?;
    println!("Response from {} is {}", URL, status_code.to_string());
    let output_filename = format!("image-{}.jpeg", chrono::Utc::now().format("%H%M%S"));
    let exe_path = env::current_exe().unwrap();
    let base_dir = exe_path
        .ancestors()
        .nth(3)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        + "\\data\\";
    let mut out_file = File::create(base_dir.to_owned() + &output_filename)?;
    out_file.write_all(&img_data)?;

    println!("Image saved to '{}'", output_filename);
    Ok(())
}
