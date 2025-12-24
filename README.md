🚀 OxideBlog
 not completed yet ..........
 
  
OxideBlog is a fast, minimalist, file-based blog engine built with Rust and Actix-web.
It is designed for developers who value performance, simplicity, and full control over their content.

✨ Features

🦀 Written in Rust

⚡ Powered by Actix-web

📝 Markdown blog posts

📄 TOML frontmatter support

🔍 Automatic post discovery

🧱 Clean, extensible architecture

🚫 No database — filesystem based

🛠 Tech Stack

Language: Rust

Web Framework: Actix-web

Content Format: Markdown

Metadata: TOML Frontmatter

Build Tool: Cargo

📁 Project Structure
oxideblog/
├── src/
│   ├── main.rs
│   ├── handlers/
│   │   ├── home_handler.rs
│   │   └── post_handler.rs
│   ├── models/
│   │   └── frontmatter.rs
│   └── utils/
├── posts/
│   └── example-post.md
├── static/
│   └── style.css
├── templates/
│   ├── index.html
│   └── post.html
├── Cargo.toml
└── README.md

📝 Creating a Blog Post

Add a Markdown file inside the posts/ directory.

+++
title = "My First Post"
date = "2025-01-01"
author = "Vivek Atkari"
description = "Introduction to OxideBlog"
+++

# Hello World 👋

This blog is powered by **OxideBlog**, a fast and minimal Rust blog engine.

▶️ Running the Project
git clone https://github.com/Vivek23456/blog-site.git
cd blog-site
cargo run


Open in browser:

http://127.0.0.1:8080

🔄 Updating the Blog
git add .
git commit -m "Update blog content"
git push
