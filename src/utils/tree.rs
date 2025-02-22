use {
    super::style_code,
    itertools::Itertools,
    maud::{html, Markup, PreEscaped},
    pulldown_cmark::Options,
    std::{collections::HashMap, sync::LazyLock},
    wini_macros::page,
};

#[derive(Debug, serde::Deserialize)]
pub enum PageOrDirectory<'l> {
    Page {
        title: &'l str,
        page: &'l str,
    },
    Directory {
        is_ordered: bool,
        title: &'l str,
        page: Option<&'l str>,
        pages: Vec<PageOrDirectory<'l>>,
    },
}

#[derive(Debug)]
enum VecOrStr<'l> {
    Vec(Vec<&'l str>),
    Str(&'l str),
}

impl<'l> PageOrDirectory<'l> {
    fn rec_get_pages(&self) -> VecOrStr {
        match self {
            PageOrDirectory::Page { page, .. } => VecOrStr::Str(page),
            PageOrDirectory::Directory { pages, page, .. } => {
                let mut final_pages = page.map_or_else(Default::default, |p| vec![p]);
                for page in pages {
                    match page.rec_get_pages() {
                        VecOrStr::Str(s) => final_pages.push(s),
                        VecOrStr::Vec(v) => final_pages.extend(v),
                    }
                }
                VecOrStr::Vec(final_pages)
            },
        }
    }

    pub fn rec_get_page_titles(&self) -> HashMap<&str, &str> {
        println!("{self:#?}");
        match self {
            PageOrDirectory::Page { title, page } => HashMap::from_iter([(*page, *title)]),
            PageOrDirectory::Directory {
                pages, page, title, ..
            } => {
                let mut final_pages = HashMap::new();

                if let Some(page) = page {
                    final_pages.insert(*page, *title);
                }

                for page in pages {
                    final_pages.extend(page.rec_get_page_titles());
                }
                final_pages
            },
        }
    }

    pub fn rec_display(&self) -> Markup {
        match self {
            PageOrDirectory::Page { title, page } => {
                html! {
                    li.cursor
                        hx-get={"/htmx/" (page)}
                        "hx-on::after-request"="hlCurrentPage()"
                        hx-target="#main"
                        hx-replace-url={"/lua/" (page)}
                    { (title) }
                }
            },
            PageOrDirectory::Directory {
                pages, page, title, ..
            } => {
                html! {
                    @if let Some(page) = page {
                        li.cursor
                            hx-get={"/htmx/" (page)}
                            "hx-on::after-request"="hlCurrentPage()"
                            hx-target="#main"
                            hx-replace-url={"/lua/" (page)}
                        { (title) }
                    } @else {
                        li { (title) }
                    }
                    ol {
                        @for page in pages {
                            (page.rec_display())
                        }
                    }
                }
            },
        }
    }

    pub fn get_nearest_pages(&self, page: &str) -> (Option<String>, Option<String>) {
        let pages = self.rec_get_pages();

        if let VecOrStr::Vec(v) = pages {
            if let Some(index_at_page) = v.iter().position(|p| *p == page) {
                if index_at_page == 0 {
                    (None, v.get(index_at_page + 1).map(|e| (*e).to_owned()))
                } else {
                    (
                        v.get(index_at_page - 1).map(|e| (*e).to_owned()),
                        v.get(index_at_page + 1).map(|e| (*e).to_owned()),
                    )
                }
            } else {
                (None, None)
            }
        } else {
            (None, None)
        }
    }
}

fn search_file_recursively<P: AsRef<std::path::Path>>(
    dir: P,
    target_name: &str,
) -> std::io::Result<Option<String>> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(found) = search_file_recursively(path.to_str().unwrap(), target_name)? {
                return Ok(Some(found));
            }
        } else if path.is_file() && path.file_name().map_or(false, |name| name == target_name) {
            return Ok(Some(std::fs::read_to_string(path).unwrap()));
        }
    }

    Ok(None)
}


/// # Panics
/// Since this code will only be called on a LazyLock, it's ok if it panics
pub fn compute_pages<P: AsRef<std::path::Path> + std::fmt::Display>(
    path: P,
) -> HashMap<String, String> {
    let ron_content =
        std::fs::read_to_string(format!("{}/structure.ron", path.to_string())).unwrap();
    let page_structure: PageOrDirectory = ron::from_str(&ron_content).unwrap();

    match page_structure.rec_get_pages() {
        VecOrStr::Vec(v) => {
            v.iter()
                .map(|page| {
                    println!("{}&  {}", path.to_string(), page);
                    let file_content =
                        search_file_recursively(path.as_ref(), &format!("{page}.md"))
                            .unwrap()
                            .unwrap();

                    let parser = pulldown_cmark::Parser::new_ext(&file_content, Options::all());
                    let mut html_output = String::new();

                    pulldown_cmark::html::push_html(&mut html_output, parser);

                    let clone = html_output.clone();
                    let mut dom = tl::parse(&clone, tl::ParserOptions::default())
                        .expect("HTML string too long");

                    let mut code_blocks = dom
                        .query_selector("pre")
                        .expect("Failed to parse query selector")
                        .collect_vec();

                    let mut added_chars: isize = 0;
                    let mut handled = 0;

                    while code_blocks.len() > 0 {
                        let code_block = code_blocks[0];
                        let parser_mut = dom.parser();
                        let pre_code = code_block.get(parser_mut).expect("Failed to resolve node");
                        let code = pre_code.children().unwrap().all(parser_mut)[0].clone();

                        let styled = style_code::style_code(&code, parser_mut);

                        let boundaries = pre_code.as_tag().unwrap().boundaries(parser_mut);

                        let start: usize = (boundaries.0 as isize + added_chars) as usize;
                        let end: usize = (boundaries.1 as isize + added_chars) as usize;

                        html_output.replace_range(start..=end, &styled);
                        added_chars +=
                            (styled.len() as isize) - (boundaries.1 + 1 - boundaries.0) as isize;

                        handled += 1;

                        dom = tl::parse(&clone, tl::ParserOptions::default())
                            .expect("HTML string too long");

                        code_blocks = dom
                            .query_selector("pre")
                            .expect("Failed to parse query selector")
                            .skip(handled)
                            .collect_vec();
                    }



                    ((*page).to_owned(), html_output)
                })
                .collect()
        },
        VecOrStr::Str(_) => panic!("Should not occur"),
    }
}
