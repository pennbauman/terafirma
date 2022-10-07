# Terafirma
Minimal but powerful static site generator using the Tera template format


## Usage

	terafirma [OPTIONS] [COMMAND]

### Commands
 - `build`: Build static site, default command if unspecified
 - `clean`: Clean up already build site
 - `new`: Create new configuration file in the current directory
 - `help`: Print this message or the help of the given subcommand(s)

### Options
 - `-c`, `--config <FILE>`: Select custom config file, the default is 'Terafirma.toml'
 - `-h`, `--help`: Print help information
 - `-V`, `--version`: Print version information


## Terafirma.toml
This file is the central configuration that determines how a site is built. An example configuration is available here [`Terafirma.toml`](Terafirma.toml).

Files are created for the final sites form 4 source: static files, page files, `[[page]]` configuration sections, and `[[redirect]]` configuration sections. Static files are simply copied into the final site's directory with the same relative path as their source file. Page files and the `body` elements `[[page]]` configuration sections are interpreted as Tera templates and their output is place in the final site either with the same relative path as their source file or based on the `path` element of the configuration section. Redirects are created in the final site using the provided `url` and placed according to the `path` element of their configuration.

### Context
Context for Tera templates can be provided either globally or on a page by page basis. To set context globally, place values in the `[context]` section of `Terafirma.toml`. To set context for a single page, place values in the `context` element of the relevant `[[page]]` section. Page by page context will override global context if they conflict.

### Settings
The `[settings]` section can be used to change the following directories Terafirma uses:

- `output_dir`: sets the directory the final site is build in (default "output")
- `template_dir`: sets the directory containing Tera templates pages can reference (default "templates")
- `static_dir`: sets the directory containing static files (default "static")
- `page_dir`: sets the directory containing Tera template for site pages (default "pages")


### Examples
This section in a `Terafirma.toml` file will produce a simple page containing `<p>foo bar</p>` in the file `/text.html` within the final site.

	[[page]]
	path = "/text.html"
	body = "<p>foo bar</p>"

This section in a `Terafirma.toml` with cause `/pointer.html` to redirect to `https://deref.net` in the final site.

	[[redirect]]
	path = "/pointer.html"
	url = "https://deref.net"

