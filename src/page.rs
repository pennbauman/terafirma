// Terafirma static site generator
//   Penn Bauman <me@pennbauman.com>
use std::fs;
use std::io::{Read, Write};
use std::path::{Path};
use anyhow::{anyhow, bail, Result};
use tera::{Tera, Context};


static REDIRECT_TEMPLATE: &str = "<!DOCTYPE html><html>
	<head><meta http-equiv='refresh' content='0; url=\"{{ url }}\"'/></head>
	<body><p><a href='{{ url }}'>{{ url }}</a></p></body>
</html>";


#[derive(Debug)]
pub enum PageBuilder {
    Redirect { path: String, url: String },
    WithoutBody { path: String, context: Context },
    TextBody { path: String, context: Context, body: String },
    FileBody { path: String, context: Context, body: String },
}
impl PageBuilder {
    pub fn path(&self) -> &str {
        match self {
            Self::Redirect{ path, url:_ } => &path,
            Self::WithoutBody{ path, context:_ } => &path,
            Self::TextBody{ path, context:_, body:_ } => &path,
            Self::FileBody{ path, context:_, body:_ } => &path,
        }
    }
    pub fn redirect(path: &str, url: &str) -> Self {
        Self::Redirect {
            path: path.to_string(),
            url: url.to_string(),
        }
    }
    pub fn no_body(path: &str, context: Context) -> Self {
        Self::WithoutBody {
            path: path.to_string(),
            context: context,
        }
    }
    pub fn text_body(path: &str, context: Context, body: &str) -> Self {
        Self::TextBody {
            path: path.to_string(),
            context: context,
            body: body.to_string(),
        }
    }
    pub fn file_body(path: &str, body: &str) -> Self {
        Self::FileBody {
            path: path.to_string(),
            context: Context::new(),
            body: body.to_string(),
        }
    }
    pub fn add_file_body(&mut self, body_file: &str) -> Result<()> {
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
    pub fn build(&self, tera: &mut Tera, global: &Context, output: &Path, pages: &Path) -> Result<()> {
        let text = self.render(tera, global, pages)?;
        //println!("Page: {}", text);
        let full_path = output.join(self.path());
        match full_path.parent() {
            Some(p) => fs::create_dir_all(p)?,
            None => (),
        };
        let mut file = fs::File::create(full_path).unwrap();
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


