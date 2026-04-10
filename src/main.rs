use std::fs;
use pulldown_cmark::{Parser, Options, html};
use tera::{Tera, Context}; 

const CONTENT_DIR: &str = "content";
const PUBLIC_DIR: &str = "public";
const STATIC_DIR: &str = "static";
const TEMPLATES_DIR: &str = "templates";
const PAGE_TEMPLATE: &str = "page.html";
const THEMES_DIR: &str = "themes";
const DEFAULT_THEME: &str = "style";

fn main() {

    //initialize the tera engine
    let tera = Tera::new(&format!("{}/**/*.html", TEMPLATES_DIR)).expect("template syntax error");

    // create the templates directory
    fs::create_dir_all(TEMPLATES_DIR).expect("Failed to create templates directory");
    println!("Templates file is created");

    // create the public directory
    if let Err(err) = fs::remove_dir_all(PUBLIC_DIR) {
        if err.kind() != std::io::ErrorKind::NotFound {
            panic!("Failed to clear public directory: {}", err);
        }
    }
    fs::create_dir_all(PUBLIC_DIR).expect("Failed to create public directory");
    println!("Puclic file is created");
    
    // copy the style.css to the public directory
    let selected_theme = read_theme_from_args();
    let source_css = theme_source_path(&selected_theme);
    let target_css = format!("{}/style.css", PUBLIC_DIR);

    if fs::metadata(&source_css).is_err() {
        eprintln!(
            "Theme '{}' not found at '{}'. Fallback to '{}'.",
            selected_theme, source_css, DEFAULT_THEME
        );
        let fallback = theme_source_path(DEFAULT_THEME);
        fs::copy(fallback, &target_css).expect("Failed to copy fallback theme css");
    } else {
        println!("Using theme: {}", selected_theme);
        fs::copy(&source_css, &target_css).expect("Failed to copy theme css");
    }

    let entries = fs::read_dir(CONTENT_DIR).expect("Please create the content directory and put .md files in it");
    for entry in entries {
        let entry = entry.expect("Failed to get entry");
        let path = entry.path();

        // check if the file is a markdown file
        if path.extension().map_or(false, |ext| ext == "md") {

            // obtain the file name without the extension
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let markdown_input = fs::read_to_string(&path).expect("Failed to read markdown file");

            // turn markdown to html
            let parser = Parser::new_ext(&markdown_input, Options::empty());
            let mut html_fragment = String::new();
            html::push_html(&mut html_fragment, parser);

            let mut context = Context::new();
            context.insert("title", file_stem); 
            context.insert("content", &html_fragment); 

            // render the HTML
            let rendered = tera
                .render(PAGE_TEMPLATE, &context)
                .expect("Failed to render the HTML");

            // if source is index.md, write to public/index.html
            // otherwise write to public/<post>/index.html
            let output_path = if file_stem == "index" {
                format!("{}/index.html", PUBLIC_DIR)
            } else {
                let post_output_dir = format!("{}/{}", PUBLIC_DIR, file_stem);
                fs::create_dir_all(&post_output_dir).expect("Failed to create post output directory");
                format!("{}/index.html", post_output_dir)
            };
            fs::write(&output_path, &rendered).expect("Failed to write html file");

            println!("🚀 Generated {}", output_path);
        }

    }
}


fn read_theme_from_args() -> String {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--theme" {
            if let Some(theme) = args.next() {
                return theme;
            }
        }
    }
    DEFAULT_THEME.to_string()
}

fn theme_source_path(theme: &str) -> String {
    format!("{}/{}/{}.css", STATIC_DIR, THEMES_DIR, theme)
}