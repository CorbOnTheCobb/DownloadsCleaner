use std::fs;
use std::io;
use std::path::Path;
use std::env;

fn main() -> io::Result<()> {
    let home_dir = env::home_dir();
    let home_dir_str = &home_dir.unwrap().to_str().unwrap().to_owned();
        let downloads_dir = home_dir_str.to_owned() + "/Downloads";
    clean_directory(&downloads_dir)
}
fn clean_directory(directory: &str) -> io::Result<()> {
    let indecies = fs::read_dir(directory)?;
    let home_dir = env::home_dir();
    let home_dir_str = &home_dir.unwrap().to_str().unwrap().to_owned();
    let documents_dir = home_dir_str.to_owned() + "/Documents";
    let pictures_dir  = home_dir_str.to_owned() + "/Pictures";
    let misc_dir      = home_dir_str.to_owned() + "/Misc";
    let videos_dir    = home_dir_str.to_owned() + "/Movies";
    let music_dir     = home_dir_str.to_owned() + "/Music";

    for index in indecies {
        let index = index?;

        if index.file_type()?.is_file() {
            let source_path = index.path();
            match get_file_extension(&source_path.to_str().unwrap()) { 
                Some("png") | Some("jpg") | Some("gif") | Some("HEIC") | Some("heic") | Some("jpeg") | Some("bmp") | Some("webp") => {
                    let destination_path = pictures_dir.to_string() + "/" + source_path.file_name().unwrap().to_str().unwrap();
                    fs::rename(&source_path, &destination_path)?;
                    println!("Moved {} to {} successfully!", source_path.display(), destination_path); 
                }
                Some("pdf") | Some("pptx") | Some("docx") | Some("xlsx") => {
                    let destination_path = documents_dir.to_string() + "/" + source_path.file_name().unwrap().to_str().unwrap();
                    fs::rename(&source_path, &destination_path)?;
                    println!("Moved {} to {} successfully!", source_path.display(), destination_path);
                }                
                Some("mp4") | Some("webm") => {
                    let destination_path = videos_dir.to_string() + "/" + source_path.file_name().unwrap().to_str().unwrap();
                    fs::rename(&source_path, &destination_path)?;
                    println!("Moved {} to {} successfuly!", source_path.display(), destination_path);
                }                
                Some("mp3") | Some("wav") => {
                    let destination_path = music_dir.to_string() + "/" + source_path.file_name().unwrap().to_str().unwrap();
                    fs::rename(&source_path, &destination_path)?;
                    println!("Moved {} to {} successfuly!", source_path.display(), destination_path);
                }
                Some("download") => { println!("{} has begun downloading!", source_path.file_name().unwrap().to_str().unwrap()); }
                _ => {
                    let destination_path = misc_dir.to_string() + "/" + source_path.file_name().unwrap().to_str().unwrap();
                    fs::rename(&source_path, &destination_path)?;
                    println!("Moved {} to {} successfuly!", source_path.display(), destination_path);
                }
            }
        }
        
        else if index.file_type()?.is_dir() {
            let _ = clean_directory(&index.path().to_str().unwrap());
        }
    }
    return Ok(());
}

fn get_file_extension(file_path: &str) -> Option<&str> {
    let path = Path::new(file_path);
    path.extension().and_then(|ext| ext.to_str())
}
