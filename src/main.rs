use fs_extra;
use fs_extra::dir::CopyOptions;
use glob::glob;
use std::fs;
use std::path::Path;

fn main() {
    let dest_dir = Path::new("./dist");
    let src_dir = Path::new("./src");
    let assets_dir = src_dir.join("assets");

    fs::remove_dir_all(dest_dir).unwrap();
    fs::create_dir_all(dest_dir).unwrap();

    for entry in glob("./src/public/*").unwrap() {
        if let Ok(path) = entry {
            fs::copy(
                path.clone(),
                dest_dir.join(path.clone().strip_prefix("src/public/").unwrap()),
            )
            .unwrap();
        }
    }

    fs_extra::dir::copy(assets_dir, dest_dir, &CopyOptions::new()).unwrap();

    for entry in glob("./src/pages/*").unwrap() {
        if let Ok(path) = entry {
            println!("{}", path.display())
            if path.extension()
        }
    }
}
