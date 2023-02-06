use fs_extra;
use fs_extra::dir::CopyOptions;
use glob::glob;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tera::{Context, Tera};

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

    let tera = Tera::new("src/**/*.tera").unwrap();

    let time_stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    let mut context = Context::new();
    context.insert("siteBasePath", "https://araf.aljami.me/");
    context.insert("timeStamp", &time_stamp);
    context.insert("siteTitle", "Araf Al Jami");
    context.insert("siteDescription", "Araf Al-Jami's personal blog");

    context.insert("body", "Araf Al-Jami's personal blog");

    for tpl in tera.get_template_names() {
        if tpl.starts_with("pages/") {
            context.insert("body", &tera.render(tpl, &context).unwrap().to_string());
            let rendered_string = tera.render("layouts/default.tera", &context).unwrap();
            let mut splitted_file_path = tpl
                .strip_prefix("pages/")
                .unwrap()
                .strip_suffix(".tera")
                .unwrap()
                .split('/')
                .collect::<Vec<&str>>();
            let file_name = splitted_file_path.last().unwrap();
            let file_name_with_ext = format!(
                "{}{}",
                file_name,
                if file_name.ends_with("index") {
                    ".html"
                } else {
                    "/index.html"
                }
            );
            splitted_file_path.pop();
            let file_dir = dest_dir.join(splitted_file_path.join("/"));
            let file_path = format!("{}{}", file_dir.to_str().unwrap(), file_name_with_ext);
            fs::create_dir_all(file_dir).unwrap();
            fs::write(&file_path, rendered_string).unwrap();
        }
    }
}
