use std::{fs, io::Error};

use actix_web::{get, web, HttpResponse, Responder};
use pulldown_cmark::{html, Options, Parser};

use super::home_handler::Frontmatter;

/// Read the markdown content of a post
fn extract_markdown(post_name: &str) -> Result<String, Error> {
    let markdown = match fs::read_to_string(format!("./posts/{}/post.md", post_name)) {
        Ok(markdown) => markdown,
        Err(e) => {
            println!("{:?}", e);
            return Err(e);
        }
    };

    Ok(markdown)
}

/// Read and deserialize the frontmatter (TOML) of a post
fn extract_frontmatter(post_name: &str) -> Result<Frontmatter, Error> {
    let frontmatter_input = match fs::read_to_string(
        format!("./posts/{}/post_frontmatter.toml", post_name),
    ) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);
            return Err(e);
        }
    };

    let frontmatter = match toml::from_str(&frontmatter_input) {
        Ok(fm) => fm,
        Err(_) => {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "could not parse post frontmatter",
            ))
        }
    };

    Ok(frontmatter)
}

#[get("/posts/{post_name}")]
pub async fn post(
    tmpl: web::Data<tera::Tera>,
    post_name: web::Path<String>,
) -> impl Responder {
    let post_name = post_name.into_inner();
    let mut context = tera::Context::new();
    let options = Options::empty();

    // Load markdown
    let markdown_input = match extract_markdown(&post_name) {
        Ok(s) => s,
        Err(_) => {
            return HttpResponse::NotFound()
                .content_type("text/html")
                .body("<p>Could not find post - sorry!</p>");
        }
    };

    // Load frontmatter
    let frontmatter = match extract_frontmatter(&post_name) {
        Ok(s) => s,
        Err(_) => {
            return HttpResponse::NotFound()
                .content_type("text/html")
                .body("<p>Could not find post - sorry!</p>");
        }
    };

    // Convert markdown → HTML
    let parser = Parser::new_ext(&markdown_input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Pass data to template
    context.insert("post", &html_output);
    context.insert("meta_data", &frontmatter);

    // Render template
    match tmpl.render("post.html", &context) {
        Ok(s) => HttpResponse::Ok()
            .content_type("text/html")
            .body(s),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::NotFound()
                .content_type("text/html")
                .body("<p>Could not render post - sorry!</p>")
        }
    }
}
