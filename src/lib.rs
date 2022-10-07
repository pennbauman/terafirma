// Terafirma static site generator
//   Penn Bauman <me@pennbauman.com>
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use anyhow::{bail, Result};
use toml::{value::Table, map::Map};
use tera::{Tera, Context};

mod page;
use page::PageBuilder;

#[derive(Debug)]
pub struct SiteBuilder {
    output_dir: PathBuf,
    template_dir: String,
    static_dir: PathBuf,
    page_dir: PathBuf,

    context: Context,

    pages: Vec<PageBuilder>,
}
impl SiteBuilder {
    pub fn from_file<P: Into<PathBuf>>(file_path: P) -> Result<Self> {
        let path = file_path.into().as_path().canonicalize()?;
        let root_dir = path.parent().unwrap();
        // Parse TOML fils
        let mut file = fs::File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data: toml::Value = toml::from_str(&contents)?;
        //println!("{:?}", data);
        // Extract settings from TOML
        let settings: Table = match data.get("settings") {
            Some(v) => match v.as_table() {
                Some(t) => t.clone(),
                None => Map::new(),
            },
            None => Map::new(),
        };
        //println!("toml parsed");

        // Determine directory paths
        let output_path = create_path(root_dir, settings.get("output_dir"), "output");
        let template_path = create_path(root_dir, settings.get("template_dir"), "templates");
        let static_path = create_path(root_dir, settings.get("static_dir"), "static");
        let page_path = create_path(root_dir, settings.get("page_dir"), "pages");
        //println!("paths created");

        // Create return value
        let mut ret = Self {
            output_dir: output_path,
            template_dir: template_path.to_str().unwrap().to_owned() + "/**/*",
            static_dir: static_path,
            page_dir: page_path,
            context: match data.get("context") {
                Some(v) => Context::from_serialize(v)?,
                None => Context::new(),
            },
            pages: vec![],
        };
        //println!("ret created");

        // Parse redirects
        let arr = match data.get("redirect") {
            Some(v) => match v.as_array() {
                Some(a) => a.clone(),
                None => vec![],
            },
            None => vec![],
        };
        for r in arr {
            //println!("redirect: {:?}", r);
            let path = get_page_path(r.get("path"))?;
            let url = match r.get("url") {
                Some(v) => match v.as_str() {
                    Some(s) => s,
                    None => bail!("Redirect URL must be a string"),
                },
                None => bail!("All redirects require a URL"),
            };
            ret.pages.push(PageBuilder::redirect(&path, url));
        }

        // Parse pages
        let arr = match data.get("page") {
            Some(v) => match v.as_array() {
                Some(a) => a.clone(),
                None => vec![],
            },
            None => vec![],
        };
        for p in arr {
            //println!("page: {:?}", p);
            let path = get_page_path(p.get("path"))?;
            let context = match p.get("context") {
                Some(v) => Context::from_serialize(v)?,
                None => Context::new(),
            };
            match p.get("body") {
                Some(v) => match v.as_str() {
                    Some(s) => ret.pages.push(PageBuilder::text_body(&path, context, s)),
                    None => bail!("Page bodies must be strings"),
                },
                None => ret.pages.push(PageBuilder::no_body(&path, context)),
            };
        }

        return Ok(ret);
    }
    pub fn build(&mut self) -> Result<()> {
        let mut tera = Tera::new(&self.template_dir)?;

        //println!("build start ...");
        self.clean()?;

        fs::create_dir_all(&self.output_dir)?;
        for f in recursive_ls(&self.static_dir)? {
            //println!("static: {}", f);
            match Path::new(&f).parent() {
                Some(p) => fs::create_dir_all(&self.output_dir.join(p))?,
                None => (),
            };
            fs::copy(&self.static_dir.join(&f), &self.output_dir.join(&f))?;
        }
        //println!("static copied");

        //println!("{:?}", recursive_ls(&self.page_dir));
        for f in recursive_ls(&self.page_dir)? {
            let mut dup = false;
            for p in &mut self.pages {
                //println!("page: {}", f);
                if p.path() == f {
                    dup = true;
                    p.add_file_body(&f)?;
                    break;
                }
            }
            if !dup {
                self.pages.push(PageBuilder::file_body(&f, &f));
            }
        }


        for p in &self.pages {
            p.build(&mut tera, &self.context, &self.output_dir, &self.page_dir)?;
        }

        Ok(())
    }
    pub fn clean(&self) -> Result<()> {
        match fs::remove_dir_all(&self.output_dir) {
            Ok(_) => Ok(()),
            Err(e) => match e.raw_os_error() {
                Some(2) => Ok(()),
                _ => Err(e.into()),
            },
        }
    }
}


fn get_page_path(src: Option<&toml::Value>) -> Result<String> {
    let path = match src {
        Some(v) => match v.as_str() {
            Some(s) => s,
            None => bail!("Page path must be a string"),
        },
        None => bail!("Missing page path"),
    };
    if path.is_empty() {
        bail!("Page path cannot be an empty string");
    }
    if path.starts_with("/") {
        return Ok(path.strip_prefix("/").unwrap().to_string());
    } else {
        bail!("Page path must start with '/'");
    }
}

fn create_path(root: &Path, val: Option<&toml::Value>, default: &str) -> PathBuf {
    let dir = match val {
        Some(v) => match v.as_str() {
            Some(s) => s,
            None => return root.join(default),
        },
        None => return root.join(default),
    };
    let mut path = PathBuf::from(dir);
    if !path.is_absolute() {
        path = root.join(path);
    }
    return path.to_path_buf();
}

fn recursive_ls(dir: &Path) -> Result<Vec<String>> {
    let mut ret = vec![];
    if !dir.is_dir() {
        if dir.is_file() {
            panic!("recursive_ls() requires a directory");
        }
        return Ok(ret);
    }
    let mut subdirs = vec![PathBuf::from("")];
    while subdirs.len() > 0 {
        let prefix = subdirs.pop().unwrap();
        for entry in fs::read_dir(dir.join(&prefix))? {
            //println!("{:?}", entry);
            match entry {
                Ok(e) => {
                    let kind = e.file_type()?;
                    if kind.is_dir() {
                        subdirs.push(prefix.join(e.file_name()));
                    } else if kind.is_file() {
                        match prefix.join(e.file_name()).to_str() {
                            Some(s) => ret.push(s.to_string()),
                            None => (),
                        };
                    }
                },
                Err(e) => return Err(e.into()),
            };
        }
    }
    return Ok(ret);
}
