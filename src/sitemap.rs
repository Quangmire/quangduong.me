use std::fmt;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
pub enum SiteMapUpdateFreq {
    NEVER,
    YEARLY,
    MONTHLY,
    WEEKLY,
    DAILY,
    HOURLY,
    ALWAYS,
}

impl fmt::Display for SiteMapUpdateFreq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct StaticFilesEntry {
    url: String,
    last_mod: String,
    update_freq: String,
}

struct SiteMapEntry {
    url: String,
    last_mod: NaiveDateTime,
    update_freq: SiteMapUpdateFreq,
}

pub struct SiteMap {
    entries: Vec<SiteMapEntry>,
    sf_entries: HashMap<String, StaticFilesEntry>,
}

impl SiteMap {
    pub fn new(static_files_yaml_path: &PathBuf) -> SiteMap {
        let static_files_yaml = std::fs::File::open(static_files_yaml_path).unwrap();
        SiteMap {
            entries: Vec::new(),
            sf_entries: serde_yaml::from_reader(static_files_yaml).unwrap(),
        }
    }

    pub fn add_entry(&mut self, url: String, last_mod: NaiveDateTime, update_freq: SiteMapUpdateFreq) {
        self.entries.push(SiteMapEntry {
            url: url,
            last_mod: last_mod,
            update_freq: update_freq,
        });
    }

    pub fn write(&self, sitemap_path: PathBuf) {
        let mut sitemap_file = match File::create(&sitemap_path) {
            Ok(file) => file,
            Err(e) => panic!("Couldn't create file {}: {}", sitemap_path.display(), e),
        };

        sitemap_file.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n").unwrap();
        sitemap_file.write_all(b"<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n").unwrap();

        for entry in self.entries.iter() {
            sitemap_file.write_all(b"  <url>\n").unwrap();
            sitemap_file.write_all(format!("    <loc>https://quangduong.me/{}</loc>\n", &entry.url).as_bytes()).unwrap();
            sitemap_file.write_all(format!("    <lastmod>{}</lastmod>\n", &entry.last_mod.format("%Y-%m-%d").to_string()).as_bytes()).unwrap();
            sitemap_file.write_all(format!("    <changefreq>{}</changefreq>\n", &entry.update_freq.to_string()).as_bytes()).unwrap();
            sitemap_file.write_all(b"  </url>\n").unwrap();
        }
        for (_, entry) in self.sf_entries.iter() {
            sitemap_file.write_all(b"  <url>\n").unwrap();
            sitemap_file.write_all(format!("    <loc>https://quangduong.me/{}</loc>\n", &entry.url).as_bytes()).unwrap();
            sitemap_file.write_all(format!("    <lastmod>{}</lastmod>\n", &entry.last_mod).as_bytes()).unwrap();
            sitemap_file.write_all(format!("    <changefreq>{}</changefreq>\n", &entry.update_freq).as_bytes()).unwrap();
            sitemap_file.write_all(b"  </url>\n").unwrap();
        }
        sitemap_file.write_all(b"</urlset>\n").unwrap();
    }
}
