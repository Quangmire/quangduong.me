use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::cmp::Ordering;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct PostMetaData {
    pub title: String,
    pub path: String,
    pub card_title: String,
    pub card_date: String,
    pub last_updated: Option<String>,
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
    pub last_updated: NaiveDateTime,
}

#[derive(Serialize)]
pub struct Tag<'a> {
    class: &'a str,
    name: &'a str,
}

#[derive(Serialize)]
pub struct ArchiveTag<'a> {
    class: &'a str,
    name: &'a str,
    num: usize,
}

#[derive(Serialize)]
pub struct Base<'a> {
    // base.html
    summary: &'a str,
    needs_latex: bool,

    // navbar_base.html
    title: &'a str,
}

#[derive(Serialize)]
pub struct Post<'a> {
    base: Base<'a>,
    tags: Vec<Tag<'a>>,
    title: &'a str,
    date: &'a str,
    last_updated: &'a str,
    class: &'a str,
    newer_path: &'a str,
    older_path: &'a str,
    content: &'a str,
}

#[derive(Serialize)]
pub struct Pagination<'a> {
    path: &'a str,
    tag: &'a str,
    cur_page: usize,
    pages: Vec<usize>,
    num_pages: usize,
}

#[derive(Serialize)]
pub struct SummaryCard<'a> {
    path: &'a str,
    title: &'a str,
    date: &'a str,
    tags: Vec<Tag<'a>>,
    summary: &'a str,
}

#[derive(Serialize)]
pub struct MultiPost<'a> {
    base: Base<'a>,
    pagination: Pagination<'a>,
    summary_cards: Vec<SummaryCard<'a>>,
    page_title: &'a str,
    header_card: &'a str,
}

#[derive(Serialize)]
pub struct ArchivePost<'a> {
    title: &'a str,
    path: &'a str,
    year: &'a str,
    month: &'a str,
}

#[derive(Serialize)]
pub struct Archive<'a> {
    base: Base<'a>,
    tags: Vec<ArchiveTag<'a>>,
    posts: Vec<ArchivePost<'a>>,
}

#[derive(Serialize)]
pub struct PageNotFound<'a> {
    base: Base<'a>
}

fn is_primary(tag: &str) -> bool {
    tag == "blog" || tag == "projects" || tag == "notes" || tag == "note" || tag == "about"
}

fn tag_class(tag: &str) -> &str {
    if is_primary(tag) {
        "primary-tag"
    } else {
        "default-tag"
    }
}

pub fn page_not_found<'a>() -> PageNotFound<'a> {
    PageNotFound {
        base: Base {
            summary: "404 - Page Not Found",
            needs_latex: false,
            title: "",
        }
    }
}

pub fn generate_post<'a>(post: &'a PostData, older_post: &'a str, newer_post: &'a str) -> Post<'a> {
    Post {
        base: Base {
            summary: &post.metadata.summary,
            needs_latex: post.needs_latex,
            title: &post.metadata.title,
        },
        tags: post.metadata.card_tags.iter().map(|name| 
            Tag {
                class: tag_class(name),
                name: name,
            }
        ).collect(),
        title: &post.metadata.card_title,
        class: &post.metadata.card_class,
        date: &post.metadata.card_date,
        last_updated: match &post.metadata.last_updated {
            Some(s) => s,
            _ => "",
        },
        newer_path: newer_post,
        older_path: older_post,
        content: &post.html,
    }
}

fn compute_pages_shown(cur_page: usize, num_pages: usize) -> Vec<usize> {
    let mut pages = Vec::new();

    // Compute start page
    let mut start_page: usize = 1;
    if cur_page > start_page + 2 {
        start_page = cur_page - 2;
    }
    if start_page + 4 > num_pages && num_pages >= 5 {
        start_page = num_pages - 4;
    }

    // Compute last page
    let end_page = if start_page + 5 > num_pages + 1 {
        num_pages + 1
    } else {
        start_page + 5
    };

    // Show ellipse if first page not shown
    if start_page != 1 {
        pages.push(0);
    }

    // Add intermediate pages
    for page in start_page..end_page {
        pages.push(page);
    }


    // Show ellipse if last page not shown
    if pages[pages.len() - 1] != num_pages {
        pages.push(0);
    }

    pages
}

pub fn generate_multipost<'a>(
        post_data: &'a Vec<&PostData>,
        multipost_summary: &'a str, multipost_title: &'a str, multipost_header: &'a str,
        pagination_path: &'a str, pagination_tag: &'a str, cur_page: usize, num_pages: usize, posts_per_page: usize,
        ) -> MultiPost<'a> {
    let pages = compute_pages_shown(cur_page, num_pages);
    let start_post = (cur_page - 1) * posts_per_page;
    let end_post = if start_post + posts_per_page > post_data.len() {
        post_data.len()   
    } else {
        start_post + posts_per_page
    };
    MultiPost {
        base: Base {
            summary: multipost_summary,
            needs_latex: false,
            title: multipost_title,
        },
        pagination: Pagination {
            path: pagination_path,
            tag: pagination_tag,
            cur_page: cur_page,
            pages: pages,
            num_pages: num_pages,
        },
        summary_cards: (start_post..end_post).into_iter().map(|page|
            SummaryCard {
                path: &post_data[page].metadata.path,
                title: &post_data[page].metadata.card_title,
                date: &post_data[page].metadata.card_date,
                tags: post_data[page].metadata.card_tags.iter().map(|name| 
                    Tag {
                        class: tag_class(name),
                        name: name,
                    }
                ).collect(),
                summary: &post_data[page].metadata.summary,
            },
        ).collect(),
        page_title: multipost_title,
        header_card: multipost_header,
    }
}

pub fn generate_archive<'a>(post_data: &'a Vec<&PostData>, tag_counts: &'a HashMap<&String, usize>) -> Archive<'a> {
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
            summary: "Archive of all posts on Quang Duong's academic blog about Computer Science, Machine Learning, and Computer Architecture research",
            needs_latex: false,
            title: "",
        },
        posts: post_data.iter().map(|post|
            ArchivePost {
                title: &post.metadata.card_title,
                path: &post.metadata.path,
                year: &post.year,
                month: &post.month,
            }
        ).collect(),
        tags: tag_counts_vec.iter().map(|(tag, num)|
            ArchiveTag {
                class: tag_class(tag),
                name: tag,
                num: **num,
            }
        ).collect(),
    }
}
