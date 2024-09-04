// Copyright 2015 Google Inc. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! HTML renderer that takes an iterator of events as input.

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant,Occupied};
//use std::io::{self, Write};
use std::io;

use slug::slugify;

//use pulldown_cmark::escape::{escape_href, escape_html, StrWrite, WriteWrapper};
use pulldown_cmark::escape::{escape_href, escape_html, StrWrite};
use pulldown_cmark::Event::*;
use pulldown_cmark::{Alignment, CodeBlockKind, Event, LinkType, Tag, HeadingLevel};
use pulldown_cmark::CowStr;

use syntect::parsing::{SyntaxSet, SyntaxReference};
use syntect::highlighting::{ThemeSet, Theme};
use syntect::html::{start_highlighted_html_snippet, append_highlighted_html_for_styled_line, IncludeBackground};
use syntect::easy::HighlightLines;
use syntect::util::LinesWithEndings;
use syntect::Error;

enum TableState {
    Head,
    Body,
}

struct HtmlWriter<'a, I, W> {
    /// Iterator supplying events.
    iter: I,

    /// Writer to write to.
    writer: W,

    /// Whether or not the last write wrote a newline.
    end_newline: bool,

    table_state: TableState,
    table_alignments: Vec<Alignment>,
    table_cell_index: usize,
    footnote_numbers: HashMap<CowStr<'a>, usize>,
    figure_numbers: HashMap<String, usize>,
    footnote: bool,
    prev_raw_text: String,
    figure_link: bool,
    alt_text: bool,
    heading: bool,
    heading_level: HeadingLevel,

    in_code_block: bool,
    code_lang: String,
    syntax_set: SyntaxSet,
    theme: Theme,
}

impl<'a, I, W> HtmlWriter<'a, I, W>
where
    I: Iterator<Item = Event<'a>>,
    W: StrWrite,
{
    fn new(iter: I, writer: W) -> Self {
        Self {
            iter,
            writer,
            end_newline: true,
            table_state: TableState::Head,
            table_alignments: vec![],
            table_cell_index: 0,
            footnote_numbers: HashMap::new(),
            figure_numbers: HashMap::new(),
            footnote: false,
            prev_raw_text: "".to_string(),
            figure_link: false,
            alt_text: false,
            heading: false,
            heading_level: HeadingLevel::H1,
            in_code_block: false,
            code_lang: String::from(""),
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme: ThemeSet::get_theme("dracula.tmTheme").unwrap(),
        }
    }

    /// Writes a new line.
    fn write_newline(&mut self) -> io::Result<()> {
        self.end_newline = true;
        self.writer.write_str("\n")
    }

    /// Writes a buffer, and tracks whether or not a newline was written.
    #[inline]
    fn write(&mut self, s: &str) -> io::Result<()> {
        self.writer.write_str(s)?;

        if !s.is_empty() {
            self.end_newline = s.ends_with('\n');
        }
        Ok(())
    }

    // Modified from syntect
    pub fn highlighted_html_for_string(
        s: &str,
        ss: &SyntaxSet,
        syntax: &SyntaxReference,
        theme: &Theme,
    ) -> Result<String, Error> {
        let mut highlighter = HighlightLines::new(syntax, theme);
        let (mut output, bg) = start_highlighted_html_snippet(theme);
        output.push_str("<code class='highlight'>");

        for line in LinesWithEndings::from(s) {
            let regions = highlighter.highlight_line(line, ss)?;
            append_highlighted_html_for_styled_line(
                &regions[..],
                IncludeBackground::IfDifferent(bg),
                &mut output,
            )?;
        }
        output.push_str("</code></pre>\n");
        Ok(output)
    }


    pub fn run(mut self) -> io::Result<()> {
        while let Some(event) = self.iter.next() {
            match event {
                Start(tag) => {
                    self.start_tag(tag)?;
                }
                End(tag) => {
                    self.end_tag(tag)?;
                }
                Text(text) => {
                    if self.in_code_block {
                        /*
                        for s in ss.syntaxes() {
                            println!("{:?}", &s.name);
                        }
                        */
                        let code_string = Self::highlighted_html_for_string(&text, &self.syntax_set, self.syntax_set.find_syntax_by_name(&self.code_lang).unwrap(), &self.theme).unwrap();
                        write!(&mut self.writer, "{}", &code_string)?;
                        self.end_newline = text.ends_with('\n');
                    } else {
                        if self.heading {
                            if self.end_newline {
                                self.end_newline = false;
                                write!(&mut self.writer, "<{} id=\"{}\">", self.heading_level, slugify(&text))?;
                            } else {
                                write!(&mut self.writer, "\n<{} id=\"{}\">", self.heading_level, slugify(&text))?;
                            }
                            escape_html(&mut self.writer, &text)?;
                            self.end_newline = text.ends_with('\n');
                            continue;
                        }
                        if self.figure_link {
                            match self.figure_numbers.entry(text.to_string()) {
                                Occupied(entry) => {
                                    write!(&mut self.writer, "{}", entry.get())?;
                                    self.figure_link = false;
                                    continue;
                                },
                                Vacant(_) => {self.figure_link = false},
                            }
                        }
                        escape_html(&mut self.writer, &text)?;
                        self.end_newline = text.ends_with('\n');
                    }
                }
                Code(text) => {
                    self.write("<code>")?;
                    escape_html(&mut self.writer, &text)?;
                    self.write("</code>")?;
                }
                Html(html) => {
                    self.write(&html)?;
                }
                SoftBreak => {
                    self.write_newline()?;
                }
                HardBreak => {
                    self.write("<br />\n")?;
                }
                Rule => {
                    if self.end_newline {
                        self.write("<hr />\n")?;
                    } else {
                        self.write("\n<hr />\n")?;
                    }
                }
                FootnoteReference(name) => {
                    let len = self.footnote_numbers.len() + 1;
                    self.write("<sup class=\"footnote-reference\" id=\"")?;
                    escape_html(&mut self.writer, &name)?;
                    self.write("-fn\"><a href=\"#")?;
                    escape_html(&mut self.writer, &name)?;
                    let number = *self.footnote_numbers.entry(name).or_insert(len);
                    self.write("-fn-def\">")?;
                    write!(&mut self.writer, "{}", number)?;
                    self.write("</a></sup>")?;
                }
                TaskListMarker(true) => {
                    self.write("<input disabled=\"\" type=\"checkbox\" checked=\"\"/>\n")?;
                }
                TaskListMarker(false) => {
                    self.write("<input disabled=\"\" type=\"checkbox\"/>\n")?;
                }
            }
        }
        Ok(())
    }

    /// Writes the start of an HTML tag.
    fn start_tag(&mut self, tag: Tag<'a>) -> io::Result<()> {
        match tag {
            Tag::Paragraph => {
                if self.footnote {
                    self.write(" ")
                } else {
                    if self.end_newline {
                        self.write("<p>")
                    } else {
                        self.write("\n<p>")
                    }
                }
            }
            Tag::Heading(level, _, _) => {
                self.heading = true;
                self.heading_level = level;
                self.write("")
            }
            Tag::Table(alignments) => {
                self.table_alignments = alignments;
                self.write("<table>")
            }
            Tag::TableHead => {
                self.table_state = TableState::Head;
                self.table_cell_index = 0;
                self.write("<thead><tr>")
            }
            Tag::TableRow => {
                self.table_cell_index = 0;
                self.write("<tr>")
            }
            Tag::TableCell => {
                match self.table_state {
                    TableState::Head => {
                        self.write("<th")?;
                    }
                    TableState::Body => {
                        self.write("<td")?;
                    }
                }
                match self.table_alignments.get(self.table_cell_index) {
                    Some(&Alignment::Left) => self.write(" align=\"left\">"),
                    Some(&Alignment::Center) => self.write(" align=\"center\">"),
                    Some(&Alignment::Right) => self.write(" align=\"right\">"),
                    _ => self.write(">"),
                }
            }
            Tag::BlockQuote => {
                if self.end_newline {
                    self.write("<blockquote>\n")
                } else {
                    self.write("\n<blockquote>\n")
                }
            }
            Tag::CodeBlock(info) => {
                self.in_code_block = true;
                if !self.end_newline {
                    self.write_newline()?;
                }
                match info {
                    CodeBlockKind::Fenced(info) => {
                        let lang = info.split(' ').next().unwrap();
                        if lang.is_empty() {
                            panic!("Fenced code block without language");
                        }
                        self.code_lang = String::from(lang);
                    }
                    CodeBlockKind::Indented => {
                        panic!("Indented code block without language");
                    }
                }
                self.write("")
            }
            Tag::List(Some(1)) => {
                if self.end_newline {
                    self.write("<ol>\n")
                } else {
                    self.write("\n<ol>\n")
                }
            }
            Tag::List(Some(start)) => {
                if self.end_newline {
                    self.write("<ol start=\"")?;
                } else {
                    self.write("\n<ol start=\"")?;
                }
                write!(&mut self.writer, "{}", start)?;
                self.write("\">\n")
            }
            Tag::List(None) => {
                if self.end_newline {
                    self.write("<ul class='mdlist'>\n")
                } else {
                    self.write("\n<ul>\n")
                }
            }
            Tag::Item => {
                if self.end_newline {
                    self.write("<li>")
                } else {
                    self.write("\n<li>")
                }
            }
            Tag::Emphasis => self.write("<em>"),
            Tag::Strong => self.write("<strong>"),
            Tag::Strikethrough => self.write("<del>"),
            Tag::Link(LinkType::Email, dest, title) => {
                self.write("<a href=\"mailto:")?;
                escape_href(&mut self.writer, &dest)?;
                if !title.is_empty() {
                    self.write("\" title=\"")?;
                    escape_html(&mut self.writer, &title)?;
                }
                self.write("\">")
            }
            Tag::Link(_link_type, dest, title) => {
                self.write("<a href=\"")?;
                escape_href(&mut self.writer, &dest)?;
                self.figure_link = false;
                if dest.len() > 0 && dest.chars().nth(0).unwrap() == '#' {
                    self.figure_link = true;
                }
                if !title.is_empty() {
                    self.write("\" title=\"")?;
                    escape_html(&mut self.writer, &title)?;
                }
                self.write("\">")
            }
            Tag::Image(_link_type, dest, title) => {
                self.write("<img src=\"")?;
                escape_href(&mut self.writer, &dest)?;
                self.write("\" alt=\"")?;
                self.alt_text = true;
                self.raw_text()?;
                self.alt_text = false;
                let temp = self.prev_raw_text.clone();
                let split: Vec<_> = temp.split("||").collect();
                if !title.is_empty() {
                    self.write("\" title=\"")?;
                    escape_html(&mut self.writer, &title)?;
                }
                if split.len() > 1 {
                    let len = self.figure_numbers.len() + 1;
                    self.write("\" />")?;
                    self.write("<figcaption id=\"")?;
                    escape_html(&mut self.writer, &split[0])?;
                    let number = *self.figure_numbers.entry(split[0].to_string()).or_insert(len);
                    self.write("\" class=\"figure\"><strong>Figure ")?;
                    write!(&mut self.writer, "{}", number)?;
                    self.write("</strong>:")?;
                    escape_html(&mut self.writer, &split[1])?;
                    self.write("</figcaption>")
                } else {
                    self.write("\" />")
                }
            }
            Tag::FootnoteDefinition(name) => {
                if !self.footnote {
                    self.footnote = true;
                    self.write("<div class=\"footnotes\"><hr><ol>")?;
                }
                if self.end_newline {
                    self.write("<li class=\"footnote-definition\" id=\"")?;
                } else {
                    self.write("\n<li class=\"footnote-definition\" id=\"")?;
                }
                escape_html(&mut self.writer, &*name)?;
                self.write("-fn-def\">")
            }
        }
    }

    fn end_tag(&mut self, tag: Tag) -> io::Result<()> {
        match tag {
            Tag::Paragraph => {
                if self.footnote {
                    self.write(" ")?;
                } else {
                    self.write("</p>\n")?;
                }
            }
            Tag::Heading(level, _, _) => {
                self.write("</")?;
                write!(&mut self.writer, "{}", level)?;
                self.heading = false;
                self.write(">\n")?;
            }
            Tag::Table(_) => {
                self.write("</tbody></table>\n")?;
            }
            Tag::TableHead => {
                self.write("</tr></thead><tbody>\n")?;
                self.table_state = TableState::Body;
            }
            Tag::TableRow => {
                self.write("</tr>\n")?;
            }
            Tag::TableCell => {
                match self.table_state {
                    TableState::Head => {
                        self.write("</th>")?;
                    }
                    TableState::Body => {
                        self.write("</td>")?;
                    }
                }
                self.table_cell_index += 1;
            }
            Tag::BlockQuote => {
                self.write("</blockquote>\n")?;
            }
            Tag::CodeBlock(_) => {
                self.in_code_block = false;
            }
            Tag::List(Some(_)) => {
                self.write("</ol>\n")?;
            }
            Tag::List(None) => {
                self.write("</ul>\n")?;
            }
            Tag::Item => {
                self.write("</li>\n")?;
            }
            Tag::Emphasis => {
                self.write("</em>")?;
            }
            Tag::Strong => {
                self.write("</strong>")?;
            }
            Tag::Strikethrough => {
                self.write("</del>")?;
            }
            Tag::Link(_, _, _) => {
                self.write("</a>")?;
            }
            Tag::Image(_, _, _) => (), // shouldn't happen, handled in start
            Tag::FootnoteDefinition(name) => {
                self.write("<a href=\"#")?;
                escape_html(&mut self.writer, &*name)?;
                self.write("-fn\">↩</a>")?;
            }
        }
        Ok(())
    }

    // run raw text, consuming end tag
    fn raw_text(&mut self) -> io::Result<()> {
        let mut nest = 0;
        while let Some(event) = self.iter.next() {
            match event {
                Start(_) => nest += 1,
                End(_) => {
                    if nest == 0 {
                        break;
                    }
                    nest -= 1;
                }
                Html(text) | Code(text) | Text(text) => {
                    if self.alt_text {
                        let split: Vec<_> = text.split("||").collect();
                        self.prev_raw_text = text.to_string();
                        if split.len() > 1 {
                            escape_html(&mut self.writer, &split[1])?;
                            self.end_newline = text.ends_with('\n');
                        } else {
                            escape_html(&mut self.writer, &text)?;
                            self.end_newline = text.ends_with('\n');
                        }
                    } else {
                        escape_html(&mut self.writer, &text)?;
                        self.end_newline = text.ends_with('\n');
                    }
                }
                SoftBreak | HardBreak | Rule => {
                    self.write(" ")?;
                }
                FootnoteReference(name) => {
                    let len = self.footnote_numbers.len() + 1;
                    let number = *self.footnote_numbers.entry(name).or_insert(len);
                    write!(&mut self.writer, "[{}]", number)?;
                }
                TaskListMarker(true) => self.write("[x]")?,
                TaskListMarker(false) => self.write("[ ]")?,
            }
        }
        Ok(())
    }
}

/// Iterate over an `Iterator` of `Event`s, generate HTML for each `Event`, and
/// push it to a `String`.
///
/// # Examples
///
/// ```
/// use pulldown_cmark::{html, Parser};
///
/// let markdown_str = r#"
/// hello
/// =====
///
/// * alpha
/// * beta
/// "#;
/// let parser = Parser::new(markdown_str);
///
/// let mut html_buf = String::new();
/// html::push_html(&mut html_buf, parser);
///
/// assert_eq!(html_buf, r#"<h1>hello</h1>
/// <ul>
/// <li>alpha</li>
/// <li>beta</li>
/// </ul>
/// "#);
/// ```
pub fn push_html<'a, I>(s: &mut String, iter: I)
where
    I: Iterator<Item = Event<'a>>,
{
    HtmlWriter::new(iter, s).run().unwrap();
}

/*
/// Iterate over an `Iterator` of `Event`s, generate HTML for each `Event`, and
/// write it out to a writable stream.
///
/// **Note**: using this function with an unbuffered writer like a file or socket
/// will result in poor performance. Wrap these in a
/// [`BufWriter`](https://doc.rust-lang.org/std/io/struct.BufWriter.html) to
/// prevent unnecessary slowdowns.
///
/// # Examples
///
/// ```
/// use pulldown_cmark::{html, Parser};
/// use std::io::Cursor;
///
/// let markdown_str = r#"
/// hello
/// =====
///
/// * alpha
/// * beta
/// "#;
/// let mut bytes = Vec::new();
/// let parser = Parser::new(markdown_str);
///
/// html::write_html(Cursor::new(&mut bytes), parser);
///
/// assert_eq!(&String::from_utf8_lossy(&bytes)[..], r#"<h1>hello</h1>
/// <ul>
/// <li>alpha</li>
/// <li>beta</li>
/// </ul>
/// "#);
/// ```
pub fn write_html<'a, I, W>(writer: W, iter: I) -> io::Result<()>
where
    I: Iterator<Item = Event<'a>>,
    W: Write,
{
    HtmlWriter::new(iter, WriteWrapper(writer)).run()
}
*/
