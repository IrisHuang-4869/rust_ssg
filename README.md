# rust_ssg

A tiny Rust static site generator (SSG) for practicing.

This project currently focuses on two core features:
- Convert Markdown files in `content/` into HTML.
- Apply a user-defined template and custom CSS to generated pages.

## Features

- Convert `content/*.md` to `public/*.html`
- Render each page using `templates/page.html` (Tera template)
- Apply styles from `static/themes/<theme>.css` (copied to `public/style.css`)
- Support theme selection via CLI: `--theme <name>`
- Use the markdown filename as page title (e.g. `test.md` -> `test.html`)

## Requirements

- Rust toolchain (stable), with `cargo` available

## Quick Start

```bash
git clone <your-repo-url>
cd rust_ssg
cargo run
```

## Theme Selection

Default theme:

```bash
cargo run
```

Specify a theme:

```bash
cargo run -- --theme style
cargo run -- --theme your-theme
```

Theme files should be placed in:

```text
static/themes/<name>.css
```

If the selected theme does not exist, the generator falls back to `style`.

## Project Structure

```text
rust_ssg/
├── content/            # Input markdown files
├── templates/
│   └── page.html       # HTML template (Tera)
├── static/
│   └── themes/
│       └── style.css
├── public/             # Generated output
├── src/
│   └── main.rs         # Build pipeline
└── Cargo.toml
```


## License

MIT
