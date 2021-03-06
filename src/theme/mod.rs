
use super::{ErrorKind, Result};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use utils::create_file;

static SIMPLE_FAVICON: &'static [u8] = include_bytes!("simple/static/img/favicon.png");
static SIMPLE_LOGO: &'static [u8] = include_bytes!("simple/static/img/logo.png");
static SIMPLE_MAIN_CSS: &'static [u8] = include_bytes!("simple/static/css/main.css");
static SIMPLE_HIGHLIGHT_CSS: &'static [u8] = include_bytes!("simple/static/css/highlight.css");
static SIMPLE_MAIN_JS: &'static [u8] = include_bytes!("simple/static/js/main.js");
static SIMPLE_HIGHLIGHT_JS: &'static [u8] = include_bytes!("simple/static/js/highlight.js");
static SIMPLE_BASE: &'static [u8] = include_bytes!("simple/templates/base.tpl");
static SIMPLE_INDEX: &'static [u8] = include_bytes!("simple/templates/index.tpl");
static SIMPLE_POST: &'static [u8] = include_bytes!("simple/templates/post.tpl");
static SIMPLE_TAG: &'static [u8] = include_bytes!("simple/templates/tag.tpl");

/// theme object
pub struct Theme {
    root: PathBuf,
    name: String,
    favicon: Vec<u8>,
    logo: Vec<u8>,
    main_css: Vec<u8>,
    highlight_css: Vec<u8>,
    main_js: Vec<u8>,
    highlight_js: Vec<u8>,
    base: Vec<u8>,
    index: Vec<u8>,
    post: Vec<u8>,
    tag: Vec<u8>,
}

impl Theme {
    pub fn new<P: AsRef<Path>>(root: P) -> Theme {
        Theme {
            root: root.as_ref().to_owned(),
            name: String::new(),
            favicon: Vec::new(),
            logo: Vec::new(),
            main_css: Vec::new(),
            highlight_css: Vec::new(),
            main_js: Vec::new(),
            highlight_js: Vec::new(),
            base: Vec::new(),
            index: Vec::new(),
            post: Vec::new(),
            tag: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.name.clear();
        self.favicon.clear();
        self.logo.clear();
        self.main_css.clear();
        self.highlight_css.clear();
        self.main_js.clear();
        self.highlight_js.clear();
        self.base.clear();
        self.index.clear();
        self.post.clear();
        self.tag.clear();
    }

    pub fn load(&mut self, name: &str) -> Result<()> {
        debug!("loading theme: {}", name);
        let src_dir = self.root.join("_themes").join(name);
        if src_dir.exists() {
            let mut favicon_file = File::open(src_dir.join("static/img/favicon.png"))?;
            let mut logo_file = File::open(src_dir.join("static/img/logo.png"))?;
            let mut main_css_file = File::open(src_dir.join("static/css/main.css"))?;
            let mut highlight_css_file = File::open(src_dir.join("static/css/highlight.css"))?;
            let mut main_js_file = File::open(src_dir.join("static/js/main.js"))?;
            let mut highlight_js_file = File::open(src_dir.join("static/js/highlight.js"))?;
            let mut base_file = File::open(src_dir.join("templates/base.tpl"))?;
            let mut index_file = File::open(src_dir.join("templates/index.tpl"))?;
            let mut post_file = File::open(src_dir.join("templates/post.tpl"))?;
            let mut tag_file = File::open(src_dir.join("templates/tag.tpl"))?;
            self.clear();
            self.name.push_str(name);
            favicon_file.read_to_end(&mut self.favicon)?;
            logo_file.read_to_end(&mut self.logo)?;
            main_css_file.read_to_end(&mut self.main_css)?;
            highlight_css_file.read_to_end(&mut self.highlight_css)?;
            main_js_file.read_to_end(&mut self.main_js)?;
            highlight_js_file.read_to_end(&mut self.highlight_js)?;
            base_file.read_to_end(&mut self.base)?;
            index_file.read_to_end(&mut self.index)?;
            post_file.read_to_end(&mut self.post)?;
            tag_file.read_to_end(&mut self.tag)?;
        } else {
            if name == "simple" {
                self.clear();
                self.name.push_str(name);
                self.favicon.extend_from_slice(&SIMPLE_FAVICON);
                self.logo.extend_from_slice(&SIMPLE_LOGO);
                self.main_css.extend_from_slice(&SIMPLE_MAIN_CSS);
                self.highlight_css.extend_from_slice(&SIMPLE_HIGHLIGHT_CSS);
                self.main_js.extend_from_slice(&SIMPLE_MAIN_JS);
                self.highlight_js.extend_from_slice(&SIMPLE_HIGHLIGHT_JS);
                self.base.extend_from_slice(&SIMPLE_BASE);
                self.index.extend_from_slice(&SIMPLE_INDEX);
                self.post.extend_from_slice(&SIMPLE_POST);
                self.tag.extend_from_slice(&SIMPLE_TAG);
            } else {
                bail!(ErrorKind::ThemeNotFound(self.name.clone()));
            }
        }
        Ok(())
    }

    pub fn init_dir(&self) -> Result<()> {
        let dest_dir = self.root.join("_themes").join(&self.name);
        if dest_dir.exists() {
            return Ok(());
        }
        debug!("init theme({}) ...", self.name);

        let mut favicon = create_file(&dest_dir.join("static/img/favicon.png"))?;
        favicon.write_all(&self.favicon)?;

        let mut logo = create_file(&dest_dir.join("static/img/logo.png"))?;
        logo.write_all(&self.logo)?;

        let mut main_css = create_file(&dest_dir.join("static/css/main.css"))?;
        main_css.write_all(&self.main_css)?;

        let mut highlight_css = create_file(&dest_dir.join("static/css/highlight.css"))?;
        highlight_css.write_all(&self.highlight_css)?;

        let mut main_js = create_file(&dest_dir.join("static/js/main.js"))?;
        main_js.write_all(&self.main_js)?;

        let mut highlight_js = create_file(&dest_dir.join("static/js/highlight.js"))?;
        highlight_js.write_all(&self.highlight_js)?;

        let mut base = create_file(&dest_dir.join("templates/base.tpl"))?;
        base.write_all(&self.base)?;

        let mut index = create_file(&dest_dir.join("templates/index.tpl"))?;
        index.write_all(&self.index)?;

        let mut post = create_file(&dest_dir.join("templates/post.tpl"))?;
        post.write_all(&self.post)?;

        let mut tag = create_file(&dest_dir.join("templates/tag.tpl"))?;
        tag.write_all(&self.tag)?;

        Ok(())
    }

    pub fn export_static(&self) -> Result<()> {
        debug!("exporting theme({}) static ...", self.name);
        let dest_dir = self.root.join("_builds");

        let mut favicon = create_file(&dest_dir.join("static/img/favicon.png"))?;
        favicon.write_all(&self.favicon)?;

        let mut logo = create_file(&dest_dir.join("static/img/logo.png"))?;
        logo.write_all(&self.logo)?;

        let mut main_css = create_file(&dest_dir.join("static/css/main.css"))?;
        main_css.write_all(&self.main_css)?;

        let mut highlight_css = create_file(&dest_dir.join("static/css/highlight.css"))?;
        highlight_css.write_all(&self.highlight_css)?;

        let mut main_js = create_file(&dest_dir.join("static/js/main.js"))?;
        main_js.write_all(&self.main_js)?;

        let mut highlight_js = create_file(&dest_dir.join("static/js/highlight.js"))?;
        highlight_js.write_all(&self.highlight_js)?;

        Ok(())
    }
}
