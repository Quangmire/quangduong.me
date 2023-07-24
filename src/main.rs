use std::fs::{File, remove_dir_all, create_dir_all, copy, read_to_string};
use std::io::prelude::*;
use std::path::PathBuf;
use chrono::{self, NaiveDateTime};
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
mod sitemap;
use cli::{CLIArgs,parse_args};
use context::{
    PostMetaData, PostData,
    page_not_found, generate_post, generate_multipost, generate_archive,
};
use sitemap::{SiteMap, SiteMapUpdateFreq};

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
    // Copy over netlify.toml
    match copy(&args.netlify_toml_path, args.output_path.join("netlify.toml")) {
        Ok(_) => {},
        Err(e) => panic!("Couldn't copy netlify.toml to output directory: {}", e),
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

    for file in WalkDir::new(&args.favicon_path) {
        let file = file.unwrap();
        if file.file_type().is_dir() {
            continue;
        }
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

        let datetime = NaiveDateTime::parse_from_str(&metadata.card_date, "%b %d %Y %-I:%M%p").unwrap();
        let last_updated = metadata.last_updated.clone();

        post_data.push(
            PostData {
                metadata: metadata,
                html: html,
                needs_latex: full_file_split[1].contains("\\\\(") || full_file_split[1].contains("$$"),
                year: datetime.format("%Y").to_string(),
                month: datetime.format("%B").to_string(),
                last_updated: match last_updated {
                    Some(last_updated) => {
                        NaiveDateTime::parse_from_str(&last_updated, "%b %d %Y %-I:%M%p").unwrap()
                    },
                    None => {
                        datetime
                    },
                },
            }
        );
    }

    post_data.sort_by_key(|d| Reverse(NaiveDateTime::parse_from_str(&d.metadata.card_date, "%b %d %Y %-I:%M%p").unwrap()));

    for post in post_data.iter_mut() {
        post.metadata.card_date = NaiveDateTime::parse_from_str(&post.metadata.card_date, "%b %d %Y %-I:%M%p").unwrap().format("%b %d %Y").to_string();
        post.metadata.last_updated = match &post.metadata.last_updated {
            Some(v) => {
                Some(NaiveDateTime::parse_from_str(&v, "%b %d %Y %-I:%M%p").unwrap().format("%b %d %Y").to_string())
            },
            None => {
                None
            }
        }
    }

    post_data
}

fn build(args: &CLIArgs, post_data: Vec<PostData>) {
    let mut sitemap = SiteMap::new(&args.static_files_yaml_path);
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
        if post_data[i].metadata.path == "/" {
            _render(&tera, "post.html", &Context::from_serialize(
                &generate_post(
                    &post_data[i],
                    older_post,
                    newer_post,
                )).unwrap(), &args.output_path);
            sitemap.add_entry(
                "".to_string(),
                post_data[i].last_updated.clone(),
                SiteMapUpdateFreq::DAILY
            );
            _render(&tera, "post.html", &Context::from_serialize(
                &generate_post(
                    &post_data[i],
                    older_post,
                    newer_post,
                )).unwrap(), &args.output_path.join("about"));
            sitemap.add_entry(
                "about/".to_string(),
                post_data[i].last_updated.clone(),
                SiteMapUpdateFreq::DAILY
            );
        } else {
            _render(&tera, "post.html", &Context::from_serialize(
                &generate_post(
                    &post_data[i],
                    older_post,
                    newer_post,
                )).unwrap(), &args.output_path.join(&post_data[i].metadata.path));
            sitemap.add_entry(
                post_data[i].metadata.path.to_string(),
                post_data[i].last_updated.clone(),
                SiteMapUpdateFreq::MONTHLY
            );
        }
    }

    for tag in posts_by_tag.keys() {
        let num_pages = if (posts_by_tag[tag].len() / args.posts_per_page) * args.posts_per_page == posts_by_tag[tag].len() {
            posts_by_tag[tag].len() / args.posts_per_page
        } else {
            posts_by_tag[tag].len() / args.posts_per_page + 1
        };

        for page in 1..(num_pages + 1) {
            let pagination_tag = tag.to_uppercase();
            let multipost_summary = format!("Page {} of all posts tagged [{}] on Quang Duong's academic blog about Computer Science, Machine Learning, and Computer Architecture research", page, &pagination_tag);
            let multipost_title = format!("All Posts Tagged [{}] · Page {}", &pagination_tag, page);
            let multipost_header = format!("All Posts Tagged as [{}]", tag.to_uppercase());
            let pagination_path = format!("/tag/{}", tag);

            let multipost = generate_multipost(
                &posts_by_tag[tag],
                &multipost_summary, &multipost_title, &multipost_header,
                &pagination_path, &pagination_tag, page, num_pages, args.posts_per_page,
            );
            _render(&tera, "multipost.html",
                &Context::from_serialize(&multipost).unwrap(),
                &args.output_path.join("tag").join(tag.replace(" ", "_")).join(page.to_string()));
            sitemap.add_entry(
                format!("tag/{}/{}/", tag, page).to_string(),
                chrono::offset::Local::now().naive_local(),
                SiteMapUpdateFreq::WEEKLY
            );
        }
    }

    let post_data_iter: Vec<&PostData> = post_data.iter().collect();
    let num_pages = if (post_data_iter.len() / args.posts_per_page) * args.posts_per_page == post_data_iter.len() {
        post_data_iter.len() / args.posts_per_page
    } else {
        post_data_iter.len() / args.posts_per_page + 1
    };
    for page in 1..(num_pages + 1) {
        let multipost_summary = format!("Page {} of all posts on Quang Duong's academic blog about Computer Science, Machine Learning, and Computer Architecture research", page);
        let multipost_title = format!("Blog Posts - Page {} · Quang Duong", page);
        let multipost_header = "All Blog Posts";
        let pagination_path = "/blog";
        let pagination_tag = "";
        let multipost = generate_multipost(
            &post_data_iter,
            &multipost_summary, &multipost_title, multipost_header,
            pagination_path, pagination_tag, page, num_pages, args.posts_per_page,
        );
        _render(&tera, "multipost.html",
            &Context::from_serialize(&multipost).unwrap(),
            &args.output_path.join("blog").join(page.to_string()));
        sitemap.add_entry(
            format!("blog/{}/", page).to_string(),
            chrono::offset::Local::now().naive_local(),
            SiteMapUpdateFreq::WEEKLY
        );
    }

    _render(&tera, "archive.html",
        &Context::from_serialize(&generate_archive(&post_data.iter().collect(), &tag_counts)).unwrap(),
        &args.output_path.join("archive/"));
    sitemap.add_entry(
        "archive/".to_string(),
        chrono::offset::Local::now().naive_local(),
        SiteMapUpdateFreq::WEEKLY
    );

    _render(&tera, "404.html",
        &Context::from_serialize(&page_not_found()).unwrap(),
        &args.output_path.join("404.html"));

    sitemap.write(args.output_path.join("sitemap.xml"));
}

fn main() {
    let args = parse_args();
    clean(&args);
    setup(&args);
    let post_data = read_data(&args);
    build(&args, post_data);
}
