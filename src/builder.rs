// Terafirma static site generator
//   Penn Bauman <me@pennbauman.com>
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::fs;use anyhow::{anyhow, bail, Result};
use toml::{value::Table, map::Map};
use tera::{Tera, Context};


static REDIRECT_TEMPLATE: &str = "Redirecto: {{ url }}";


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
    pub fn from_file(file_path: &str) -> Result<Self> {
        let path = Path::new(file_path).canonicalize()?;
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

        match fs::remove_dir_all(&self.output_dir) {
            Ok(_) => (),
            Err(e) => if e.raw_os_error() != Some(2) {
                return Err(e.into());
            },
            //println!("rm {:?}", e),
        };
        fs::create_dir(&self.output_dir)?;
        for f in recursive_ls(&self.static_dir)? {
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
                if p.path() == f {
                    dup = true;
                    p.add_file_body(&f)?;
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
}



#[derive(Debug)]
enum PageBuilder {
    Redirect { path: String, url: String },
    WithoutBody { path: String, context: Context },
    TextBody { path: String, context: Context, body: String },
    FileBody { path: String, context: Context, body: String },
}
impl PageBuilder {
    fn path(&self) -> &str {
        match self {
            Self::Redirect{ path, url:_ } => &path,
            Self::WithoutBody{ path, context:_ } => &path,
            Self::TextBody{ path, context:_, body:_ } => &path,
            Self::FileBody{ path, context:_, body:_ } => &path,
        }
    }
    fn redirect(path: &str, url: &str) -> Self {
        Self::Redirect {
            path: path.to_string(),
            url: url.to_string(),
        }
    }
    fn no_body(path: &str, context: Context) -> Self {
        Self::WithoutBody {
            path: path.to_string(),
            context: context,
        }
    }
    fn text_body(path: &str, context: Context, body: &str) -> Self {
        Self::TextBody {
            path: path.to_string(),
            context: context,
            body: body.to_string(),
        }
    }
    fn file_body(path: &str, body: &str) -> Self {
        Self::FileBody {
            path: path.to_string(),
            context: Context::new(),
            body: body.to_string(),
        }
    }
    fn add_file_body(&mut self, body_file: &str) -> Result<()> {
        match self {
            Self::Redirect{ path, url:_ } => bail!("Page file conflicts with redirect '{}'", path),
            Self::WithoutBody{ path, context } => {
                *self = Self::FileBody { path: path.clone(), context: context.clone(), body: body_file.to_string() };
            },
            Self::TextBody{ path, context:_, body:_ } => bail!("Conflicting bodies for page '{}'", path),
            Self::FileBody{ path, context:_, body:_ } => bail!("Conflicting files for page '{}'", path),

        }
        Ok(())
    }
    fn build(&self, tera: &mut Tera, global: &Context, output: &Path, pages: &Path) -> Result<()> {
        let text = self.render(tera, global, pages)?;
        //println!("Page: {}", text);
        let mut file = fs::File::create(output.join(self.path())).unwrap();
        file.write(text.as_bytes())?;
        Ok(())
    }
    fn render(&self, tera: &mut Tera, global: &Context, pages: &Path) -> Result<String> {
        let mut global_context = global.clone();
        match self {
            Self::Redirect{ path:_, url } => {
                let mut context = Context::new();
                context.insert("url", url);
                Ok(tera.render_str(REDIRECT_TEMPLATE, &context)?)
            },
            Self::WithoutBody{ path, context:_ } => Err(anyhow!("Missing body for page '{}'", path)),
            Self::TextBody{ path:_, context, body } => {
                global_context.extend(context.clone());
                Ok(tera.render_str(body, &global_context)?)
            },
            Self::FileBody{ path:_, context, body } => {
                let mut file = fs::File::open(pages.join(&body))?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                //println!("Content: {}", contents);
                global_context.extend(context.clone());
                Ok(tera.render_str(&contents, &global_context)?)
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
