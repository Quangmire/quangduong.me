use std::fs::{File, remove_dir_all, create_dir_all, copy, read_to_string};
use std::io::prelude::*;
use std::path::PathBuf;
use chrono::NaiveDateTime;
use std::collections::HashMap;

use std::cmp::Reverse;

use tera::{Tera, Context};
use fs_extra::dir::{copy as dir_copy, CopyOptions};
use grass;
use walkdir::WalkDir;
//use pulldown_cmark::{Parser, Options, html};
use pulldown_cmark::{Parser, Options};
use serde_yaml::from_str;

mod cli;
mod context;
mod html;
use cli::{CLIArgs,parse_args};
use context::{PostMetaData, PostData, page_not_found, generate_post, generate_multipost, generate_archive};

fn clean(args: &CLIArgs) {
    if args.output_path.exists() {
        match remove_dir_all(&args.output_path) {
            Ok(_) => {},
            Err(e) => panic!("Couldn't remove output directory: {}", e),
        };
    }

    match create_dir_all(&args.output_path) {
        Ok(_) => {},
        Err(e) => panic!("Couldn't create output directory: {}", e),
    };
}

fn setup(args: &CLIArgs) {
    // Copy over CNAME file
    match copy(&args.cname_path, args.output_path.join("CNAME")) {
        Ok(_) => {},
        Err(e) => panic!("Couldn't copy CNAME to output directory: {}", e),
    };

    // Create .nojekyll file for GitHub
    match File::create(args.output_path.join(".nojekyll")) {
        Ok(_) => {},
        Err(e) => panic!("Couldn't create .nojekyll file: {}", e),
    };

    // Compile SASS
    match grass::from_path(&args.templates_path.join("index.scss").into_os_string().into_string().unwrap(), &grass::Options::default().style(grass::OutputStyle::Compressed)) {
        Ok(s) => {
            match File::create(&args.static_path.join("index.css")) {
                Ok(mut file) => {
                    match file.write_all(&s.as_bytes()) {
                        Ok(_) => {},
                        Err(e) => panic!("Failed to write to index.css file: {}", e),
                    };
                },
                Err(e) => panic!("Couldn't create index.css file: {}", e),
            };
        },
        Err(e) => panic!("Failed to compile SASS: {}", e),
    };

    // Copy over static directory
    match dir_copy(&args.static_path, &args.output_path, &CopyOptions::new()) {
        Ok(_) => {},
        Err(e) => panic!("Couldn't copy static directory to output directory: {}", e),
    };

    /*
    for fn in os.listdir('favicon'):
        shutil.copy2(src = os.path.join('favicon', fn), dst = os.path.join(args.output, fn))
    */
    for file in WalkDir::new(&args.favicon_path) {
        let file = file.unwrap();
        if file.file_type().is_dir() {
            continue;
        }
        // Copy over CNAME file
        match copy(file.path(), args.output_path.join(file.path().file_name().unwrap())) {
            Ok(_) => {},
            Err(e) => panic!("Couldn't copy favicon files to output directory: {}", e),
        };
    }
}

fn _render(tera: &Tera, template: &str, context: &Context, destination: &PathBuf) {
    let mut dest_file: File;
    match destination.extension() {
        Some(_) => {
            dest_file = match File::create(destination) {
                Ok(file) => file,
                Err(e) => panic!("Couldn't create file {}: {}", destination.display(), e),
            };
        },
        None => {
            create_dir_all(destination).unwrap();
            dest_file = match File::create(destination.join("index.html")) {
                Ok(file) => file,
                Err(e) => panic!("Couldn't create file {}: {}", destination.display(), e),
            };
        },
    }

    tera.render_to(template, context, &mut dest_file).unwrap();
}

fn read_data(args: &CLIArgs) -> Vec<PostData> {
    let mut post_data = Vec::new();
    for file in WalkDir::new(&args.posts_path) {
        let file = file.unwrap();
        if file.file_type().is_dir() {
            continue;
        }

        // Read Markdown file and parse frontmatter separately
        let full_file = read_to_string(file.path()).unwrap();
        let full_file_split: Vec<&str> = full_file.split("---").collect();
        let mut metadata: PostMetaData = from_str(full_file_split[0]).unwrap();
        metadata.path += "/";

        // Convert Markdown to HTML
        let mut options = Options::empty();
        options.insert(Options::ENABLE_FOOTNOTES);
        let parser = Parser::new_ext(full_file_split[1], options);

        let mut html = String::new();
        html::push_html(&mut html, parser);

        let datetime = NaiveDateTime::parse_from_str(&metadata.card_date, "%B %d, %Y | %I: %M %p").unwrap();

        post_data.push(
            PostData {
                metadata: metadata,
                html: html,
                needs_latex: full_file_split[1].contains("\\\\(") || full_file_split[1].contains("$$"),
                year: datetime.format("%Y").to_string(),
                month: datetime.format("%B").to_string(),
            }
        );
    }

    post_data.sort_by_key(|d| Reverse(NaiveDateTime::parse_from_str(&d.metadata.card_date, "%B %d, %Y | %I:%M %p").unwrap()));

    post_data
}

fn build(args: &CLIArgs, post_data: Vec<PostData>) {
    let tera = match Tera::new(&args.templates_path.join("*.html").into_os_string().into_string().unwrap()) {
        Ok(t) => t,
        Err(e) => panic!("Error parsing template: {}", e),
    };

    let mut posts_by_tag: HashMap<&String, Vec<&PostData>> = HashMap::new();
    let mut tag_counts: HashMap<&String, usize> = HashMap::new();
    for i in 0..post_data.len() {
        for tag in &post_data[i].metadata.card_tags {
            if !posts_by_tag.contains_key(tag) {
                posts_by_tag.insert(tag, Vec::new());
            }
            posts_by_tag.get_mut(tag).unwrap().push(&post_data[i]);
            *tag_counts.entry(tag).or_insert(0) += 1;
        }
        let mut newer_post: &str = "";
        let mut older_post: &str = "";
        if i == 0 {
            if post_data.len() - 1 != 0 {
                older_post = &post_data[i + 1].metadata.path;
            }
        } else if i == post_data.len() - 1 {
            newer_post = &post_data[i - 1].metadata.path;
        } else {
            older_post = &post_data[i + 1].metadata.path;
            newer_post = &post_data[i - 1].metadata.path;
        }
        _render(&tera, "post.html", &Context::from_serialize(
            &generate_post(
                &post_data[i],
                older_post,
                newer_post,
            )).unwrap(), &args.output_path.join(&post_data[i].metadata.path));
    }

    for tag in posts_by_tag.keys() {
        let multipost_summary = format!("All posts tagged [{}]", tag.to_uppercase());
        let multipost_title = format!("[{}] Posts", tag.to_uppercase());
        let multipost_header = format!("Posts Tagged as [{}]", tag.to_uppercase());
        let pagination_path = format!("/tag/{}", tag);
        let pagination_tag = tag.to_uppercase();
        let multiposts = generate_multipost(
            &posts_by_tag[tag],
            &multipost_summary, &multipost_title, &multipost_header,
            &pagination_path, &pagination_tag,
        );
        for i in 0..multiposts.len() {
            _render(&tera, "multipost.html",
                &Context::from_serialize(&multiposts[i]).unwrap(),
                &args.output_path.join("tag").join(tag).join((i + 1).to_string()));
        }
    }

    let post_data_iter = post_data.iter().collect();
    let multipost_summary = "Professional blog by Quang Duong about CS/ML/Comp Arch research and topics :)";
    let multipost_title = "Self-Deprecated Dev Blog";
    let multipost_header = "All Posts";
    let pagination_path = "";
    let pagination_tag = "";
    let multiposts = generate_multipost(
        &post_data_iter,
        multipost_summary, multipost_title, multipost_header,
        pagination_path, pagination_tag,
    );
    for i in 0..multiposts.len() {
        if i == 0 {
            _render(&tera, "multipost.html",
                &Context::from_serialize(&multiposts[i]).unwrap(),
                &args.output_path.join("index.html"));
        } else {
            _render(&tera, "multipost.html",
                &Context::from_serialize(&multiposts[i]).unwrap(),
                &args.output_path.join((i + 1).to_string()));
        }
    }

    _render(&tera, "archive.html",
        &Context::from_serialize(&generate_archive(&post_data.iter().collect(), &tag_counts)).unwrap(),
        &args.output_path.join("archive/"));

    _render(&tera, "404.html",
        &Context::from_serialize(&page_not_found()).unwrap(),
        &args.output_path.join("404.html"));
}

fn main() {
    let args = parse_args();
    clean(&args);
    setup(&args);
    let post_data = read_data(&args);
    build(&args, post_data);
}
