// Terafirma system tests
//   Penn Bauman <me@pennbauman.com>
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::env;
use std::process::Command;
use terafirma::SiteBuilder;


fn get_crate_exe() -> Option<PathBuf> {
    for path in env::current_exe().ok()?.ancestors() {
        if path.ends_with("deps") {
            let mut buf = path.to_path_buf();
            buf.pop();
            buf.push("terafirma");
            return Some(buf);
        }
    }
    return None;
}


#[test]
fn test_files_only() -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new("tests/files-only");
    // Build site
    assert!(Command::new(get_crate_exe().unwrap())
            .current_dir(root)
            .status().is_ok());
    // Check static files
    assert!(&root.join("output/img/emoticon.png").is_file());
    assert!(&root.join("output/style.css").is_file());
    // Check pages
    let mut file = fs::File::open(&root.join("output/home.html"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let expected = "<!DOCTYPE html><html lang=\"en\">
	<head>
		<title>Home</title>
		<link rel=\"stylesheet\" href=\"/style.css\"/>
	</head>
	<body>
		<h1>Home</h1>

		<p>
			<a href=\"/post-1\">Post #1</a>
			<br/>
			<a href=\"/post-2\">Post #2</a>
		</p>
	</body>
</html>
";
    assert!(contents == expected);

    let mut file = fs::File::open(&root.join("output/post-1.html"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let expected = "<!DOCTYPE html><html lang=\"en\">
	<head>
		<title>Post</title>
	</head>
	<body>
		<h2>Post #1: The First One</h2>

		<p>
			
	Hello world!
	<br/>
	meep morp

		</p>
	</body>
</html>
";
    assert!(contents == expected);

    let mut file = fs::File::open(&root.join("output/post-2.html"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let expected = "<!DOCTYPE html><html lang=\"en\">
	<head>
		<title>Post</title>
	</head>
	<body>
		<h2>Post #2: The Second One</h2>

		<p>
			
	<img src=\"/img/emoticon.png\"/>

		</p>
	</body>
</html>
";
    assert!(contents == expected);

    Ok(())
}
