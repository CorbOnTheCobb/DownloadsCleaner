use std::fs;

fn main() {
    let downloads_path = Path::new("/Users/corbantravis/Downloads/");
    let documents_path = Path::new("/Users/corbantravis/Documents/");
    let pictures_path  = Path::new("/Users/corbantravis/Pictures/");
    let misc_path      = Path::new("/Users/corbantravis/Misc/");
    
    let downloads_contents = fs::read_dir("/Users/corbantravis/Downloads/").unwrap();

    for entry in downloads_contents {
        let file_type = entry.unwrap().file_type();
        if file_type == "png" {
            fs::rename(entry, pictures_path + entry);
        } else {
            println!("Did nothing.");
        }
    }
}
