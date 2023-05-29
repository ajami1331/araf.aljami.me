mod web_server;

use fs_extra;
use fs_extra::dir::CopyOptions;
use glob::glob;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tera::{Context, Tera};
use toml::{Table};

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

    let mut tera = Tera::new("src/partials/*.tera").unwrap();

    for entry in glob("src/partials/*.tera").unwrap() {
        if let Ok(path) = entry {
            let tpl_name = path.strip_prefix("src/").unwrap().to_str().unwrap();
            tera.add_template_file(path.clone(), Some(tpl_name))
                .unwrap();
        }
    }

    for entry in glob("src/layouts/*.tera").unwrap() {
        if let Ok(path) = entry {
            let tpl_name = path.strip_prefix("src/").unwrap().to_str().unwrap();
            tera.add_template_file(path.clone(), Some(tpl_name))
                .unwrap();
        }
    }

    for x in tera.get_template_names() {
        println!("{}", x);
    }

    let pages = glob("src/pages/**/*")
        .unwrap()
        .map(|f| {
            f.unwrap()
                .strip_prefix("src/")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>();

    let time_stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    let mut context = Context::new();
    let config_string = fs::read_to_string("config.toml").unwrap();
    let config = config_string.parse::<Table>().unwrap();

    context.insert("siteBasePath", &config["siteBasePath"].as_str());
    context.insert("timeStamp", &time_stamp);
    context.insert("siteTitle", &config["siteTitle"].as_str());
    context.insert("siteDescription", &config["siteDescription"].as_str());
    context.insert("githubRepo", &config["githubRepo"].as_str());

    context.insert("body", "Araf Al Jami's personal blog");

    for tpl in pages {
        if tpl.ends_with(".tera") {
            process_tera(context.clone(), src_dir, tpl.clone(), tera.clone(), dest_dir);
        }
        if tpl.ends_with(".md") {
            process_md(context.clone(), src_dir, tpl.clone(), tera.clone(), dest_dir);
        }
    }

    if config["runWebServer"].as_bool().unwrap() {
        web_server::serve(dest_dir, config["port"].as_integer().unwrap());
    }
}

fn process_tera(mut context: Context, src_dir: &Path, tpl: String, mut tera: Tera, dest_dir: &Path) {
    let file_string = fs::read_to_string(src_dir.join(tpl.clone())).unwrap();
    let (matter, stripped_string) = parse_and_find_content(&file_string).unwrap();
    println!("Stripped String{:?}", stripped_string);
    println!("Matter: {:?}", matter);
    let page_config = matter.unwrap();
    tera.add_raw_template(&tpl, stripped_string).unwrap();
    context = populate_context_from_page_config(context.clone(), page_config);
    context.insert("body", &tera.render(&tpl, &context).unwrap().to_string());
    let layout_name = format!("layouts/{}.tera", context.get("pageLayout").unwrap().as_str().unwrap());
    println!("{}", layout_name);
    let rendered_string = tera.render(&layout_name , &context).unwrap();
    let mut splitted_file_path = tpl
        .strip_prefix("pages/")
        .unwrap()
        .strip_suffix(".tera")
        .unwrap()
        .split('/')
        .collect::<Vec<&str>>();
    let file_name = splitted_file_path.last().unwrap();
    let file_name_with_ext = format!(
        "{}",
        if file_name.ends_with("index") {
            "index.html"
        } else {
            "/index.html"
        }
    );
    if file_name.ends_with("index") {
        splitted_file_path.pop();
    }
    let file_dir = dest_dir.join(splitted_file_path.join("/"));
    let file_path = format!("{}{}", file_dir.to_str().unwrap(), file_name_with_ext);
    fs::create_dir_all(file_dir).unwrap();
    fs::write(&file_path, rendered_string).unwrap();
}

fn process_md(mut context: Context, src_dir: &Path, tpl: String, mut tera: Tera, dest_dir: &Path) {
    let file_string = fs::read_to_string(src_dir.join(tpl.clone())).unwrap();
    let (matter, stripped_string) = parse_and_find_content(&file_string).unwrap();
    println!("Stripped String{:?}", stripped_string);
    println!("Matter: {:?}", matter);
    let page_config = matter.unwrap();
    tera.add_raw_template(&tpl, stripped_string).unwrap();
    context = populate_context_from_page_config(context.clone(), page_config);
    context.insert("body", &markdown::to_html(stripped_string).to_string());
    let layout_name = format!("layouts/{}.tera", context.get("pageLayout").unwrap().as_str().unwrap());
    println!("{}", layout_name);
    let rendered_string = tera.render(&layout_name , &context).unwrap();
    let mut splitted_file_path = tpl
        .strip_prefix("pages/")
        .unwrap()
        .strip_suffix(".md")
        .unwrap()
        .split('/')
        .collect::<Vec<&str>>();
    let file_name = splitted_file_path.last().unwrap();
    let file_name_with_ext = format!(
        "{}",
        if file_name.ends_with("index") {
            "index.html"
        } else {
            "/index.html"
        }
    );
    if file_name.ends_with("index") {
        splitted_file_path.pop();
    }
    let file_dir = dest_dir.join(splitted_file_path.join("/"));
    let file_path = format!("{}{}", file_dir.to_str().unwrap(), file_name_with_ext);
    println!("{}", file_dir.to_str().unwrap());
    fs::create_dir_all(file_dir).unwrap();
    fs::write(&file_path, rendered_string).unwrap();
}

fn populate_context_from_page_config(mut context: Context, page_config: Table) -> Context {
    let page_title = if page_config.contains_key("Title") {
        &page_config["Title"].as_str().unwrap()
    } else {
        ""
    };
    let page_layout = if page_config.contains_key("Layout") {
        &page_config["Layout"].as_str().unwrap()
    } else {
        "default"
    };
    let page_date = if page_config.contains_key("Date") {
        &page_config["Date"].as_str().unwrap()
    } else {
        ""
    };
    let page_language = if page_config.contains_key("Language") {
        &page_config["Language"].as_str().unwrap()
    } else {
        "en"
    };
    let page_tags = if page_config.contains_key("Tags") {
        &page_config["Tags"].as_str().unwrap()
    } else {
        "none"
    };
    context.insert("pageTitle", page_title);
    context.insert("pageDate", page_date);
    context.insert("pageLanguage", page_language);
    context.insert("pageTags", page_tags);
    context.insert("pageLayout", page_layout);

    return context;
}

fn find_toml_block(text: &str) -> Option<(usize, usize, usize)> {
    match text.starts_with("---\n") {
        true => {
            let slice_after_marker = &text[4..];
            let fm_end = slice_after_marker.find("---\n")?;
            Some((4, fm_end + 4, fm_end + 2 * 4))
        }
        false => None,
    }
}

pub fn parse_and_find_content(text: &str) -> Result<(Option<Table>, &str), toml::de::Error> {
    match find_toml_block(text) {
        Some((fm_start, fm_end, content_start)) => {
            let toml_str = &text[fm_start..fm_end];
            let documents = toml_str.parse::<Table>()?;

            let rest_of_text = &text[content_start..];

            Ok((Some(documents), rest_of_text))
        }
        None => Ok((None, text)),
    }
}

pub fn parse(text: &str) -> Result<Option<Table>, toml::de::Error> {
    let (matter, _) = parse_and_find_content(text)?;
    Ok(matter)
}
