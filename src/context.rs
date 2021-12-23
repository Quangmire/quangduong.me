use serde::{Serialize, Deserialize};
use std::cmp::min;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Debug)]
pub struct PostMetaData {
    pub title: String,
    pub path: String,
    pub card_title: String,
    pub card_date: String,
    pub card_tags: Vec<String>,
    pub card_class: String,
    pub summary: String,
}

#[derive(Debug)]
pub struct PostData {
    pub metadata: PostMetaData,
    pub html: String,
    pub needs_latex: bool,
    pub year: String,
    pub month: String,
}

#[derive(Serialize)]
pub struct Tag {
    class: String,
    name: String,
}

#[derive(Serialize)]
pub struct ArchiveTag {
    class: String,
    name: String,
    num: usize,
}

#[derive(Serialize)]
pub struct Base {
    // base.html
    summary: String,
    needs_latex: bool,

    // navbar_base.html
    title: String,
}

#[derive(Serialize)]
pub struct Post<'a> {
    base: Base,
    tags: Vec<Tag>,
    title: String,
    date: String,
    class: String,
    newer_path: String,
    older_path: String,
    content: &'a str,
}

#[derive(Serialize)]
pub struct Pagination {
    path: String,
    tag: String,
    cur_page: usize,
    pages: Vec<usize>,
    num_pages: usize,
}

#[derive(Serialize)]
pub struct SummaryCard {
    path: String,
    title: String,
    date: String,
    tags: Vec<Tag>,
    summary: String,
}

#[derive(Serialize)]
pub struct MultiPost {
    base: Base,
    pagination: Pagination,
    summary_cards: Vec<SummaryCard>,
    page_title: String,
    header_card: String,
}

#[derive(Serialize)]
pub struct ArchivePost {
    title: String,
    path: String,
    year: String,
    month: String,
}

#[derive(Serialize)]
pub struct Archive {
    base: Base,
    tags: Vec<ArchiveTag>,
    posts: Vec<ArchivePost>,
}

#[derive(Serialize)]
pub struct PageNotFound {
    base: Base
}

fn is_primary(tag: &str) -> bool {
    tag == "blog" || tag == "projects" || tag == "notes" || tag == "note" || tag == "about"
}

fn tag_class(tag: &str) -> String {
    if is_primary(tag) {
        String::from("primary-tag")
    } else {
        String::from("default-tag")
    }
}

pub fn page_not_found() -> PageNotFound {
    PageNotFound {
        base: Base {
            summary: "404 - Page Not Found".to_string(),
            needs_latex: false,
            title: "".to_string(),
        }
    }
}

pub fn generate_post(post: &PostData, older_post: Option<String>, newer_post: Option<String>) -> Post {
    Post {
        base: Base {
            summary: post.metadata.summary.clone(),
            needs_latex: post.needs_latex.clone(),
            title: post.metadata.title.clone(),
        },
        tags: post.metadata.card_tags.iter().map(|name| 
            Tag {
                class: tag_class(name),
                name: name.to_string(),
            }
        ).collect(),
        title: post.metadata.card_title.clone(),
        class: post.metadata.card_class.clone(),
        date: post.metadata.card_date.clone(),
        newer_path: match newer_post {
            Some(s) => s,
            None => "".to_string(),
        },
        older_path: match older_post {
            Some(s) => s,
            None => "".to_string(),
        },
        content: &post.html,
    }
}

fn paginate(num_pages: usize) -> Vec<usize> {
    let mut pages = Vec::new();
    let mut i = 0;
    while i < num_pages {
        pages.push(i);
        i += 5;
    }
    pages
}

fn compute_current_paginate(cur_page: usize, num_pages: usize) -> Vec<usize> {
    let mut pages = Vec::new();

    let mut start_page: usize = 1;
    if cur_page > start_page + 2 {
        start_page = cur_page - 2;
    }
    if start_page + 4 > num_pages && num_pages >= 5 {
        start_page = num_pages - 4;
    }

    if start_page != 1 {
        pages.push(0);
    }

    let mut end_page: usize = start_page + 5;
    if end_page > num_pages + 1 {
        end_page = num_pages + 1;
    }

    for page in start_page..end_page {
        pages.push(page);
    }

    if pages[pages.len() - 1] != num_pages {
        pages.push(0);
    }

    pages
}

pub fn generate_multipost(
        post_data: &Vec<&PostData>,
        multipost_summary: String, multipost_title: String, multipost_header: String,
        pagination_path: String, pagination_tag: String
        ) -> Vec<MultiPost> {
    let mut data = Vec::new();

    let mut cur_page_num = 1;
    let all_pages = paginate(post_data.len());
    let num_pages = all_pages.len();
    for start_post in all_pages {
        let pages = compute_current_paginate(cur_page_num, num_pages);
        let end_post = min(start_post + 5, post_data.len());
        data.push(MultiPost {
            base: Base {
                summary: multipost_summary.clone(),
                needs_latex: false,
                title: multipost_title.clone(),
            },
            pagination: Pagination {
                path: pagination_path.clone(),
                tag: pagination_tag.clone(),
                cur_page: cur_page_num,
                pages: pages.clone(),
                num_pages: num_pages,
            },
            summary_cards: (start_post..end_post).into_iter().map(|page|
                SummaryCard {
                    path: post_data[page].metadata.path.clone(),
                    title: post_data[page].metadata.card_title.clone(),
                    date: post_data[page].metadata.card_date.clone(),
                    tags: post_data[page].metadata.card_tags.iter().map(|name| 
                        Tag {
                            class: tag_class(name),
                            name: name.to_string(),
                        }
                    ).collect(),
                    summary: post_data[page].metadata.summary.clone(),
                },
            ).collect(),
            page_title: multipost_title.clone(),
            header_card: multipost_header.clone(),
        });
        cur_page_num += 1;
    }

    data
}

pub fn generate_archive(post_data: &Vec<&PostData>, tag_counts: &HashMap<&String, usize>) -> Archive {
    let mut tag_counts_vec: Vec<_> = tag_counts.iter().collect();
    tag_counts_vec.sort_by(|(a_tag, a_num), (b_tag, b_num)| {
        let a_primary = is_primary(a_tag);
        let b_primary = is_primary(b_tag);
        if a_primary && !b_primary {
            Ordering::Less
        } else if !a_primary && b_primary {
            Ordering::Greater
        } else if a_num < b_num {
            Ordering::Greater
        } else if b_num < a_num {
            Ordering::Less
        } else {
            a_tag.cmp(b_tag)
        }
    });
    Archive {
        base: Base {
            summary: "Archive of all posts on selfdeprecated.dev by Quang Duong".to_string(),
            needs_latex: false,
            title: "".to_string(),
        },
        posts: post_data.iter().map(|post|
            ArchivePost {
                title: post.metadata.card_title.clone(),
                path: post.metadata.path.clone(),
                year: post.year.clone(),
                month: post.month.clone(),
            }
        ).collect(),
        tags: tag_counts_vec.iter().map(|(tag, num)|
            ArchiveTag {
                class: tag_class(tag),
                name: tag.to_string(),
                num: **num,
            }
        ).collect(),
    }
}
