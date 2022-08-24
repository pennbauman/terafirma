// Terafirma system tests
//   Penn Bauman <me@pennbauman.com>
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::env;
use std::process::Command;


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
    let root = env::current_dir()?.join("tests/files-only");
    // Build site
    assert!(Command::new(get_crate_exe().unwrap())
            .current_dir(&root)
            .status().is_ok());
    // Check static files
    assert!(root.join("output/img/emoticon.png").is_file());
    assert!(root.join("output/style.css").is_file());
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


#[test]
fn test_toml_only() -> Result<(), Box<dyn std::error::Error>> {
    let root = env::current_dir()?.join("tests/toml-only");
    // Build site
    assert!(Command::new(get_crate_exe().unwrap())
            .current_dir(&root)
            .status().is_ok());
    // Check pages
    let file_path = root.join("output/index.html");
    //println!("{:?}", file_path);
    assert!(file_path.is_file());
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let expected = "HOME";
    assert!(contents == expected);

    let file_path = root.join("output/err/404.html");
    assert!(file_path.is_file());
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let expected = "404 : not found\n";
    assert!(contents == expected);

    let file_path = root.join("output/github.html");
    assert!(file_path.is_file());
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let expected = "<!DOCTYPE html><html>
	<head><meta http-equiv='refresh' content='0; url=\"https://github.com/pennbauman/terafirma\"'/></head>
	<body><p><a href='https://github.com/pennbauman/terafirma'>https://github.com/pennbauman/terafirma</a></p></body>
</html>";
    assert!(contents == expected);

    Ok(())
}

#[test]
fn test_toml_files() -> Result<(), Box<dyn std::error::Error>> {
    let root = env::current_dir()?.join("tests/toml-files");
    // Build site
    assert!(Command::new(get_crate_exe().unwrap())
            .current_dir(&root)
            .status().is_ok());
    // Check pages
    let mut file = fs::File::open(&root.join("output/post-1.html"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let expected = "<!DOCTYPE html><html lang=\"en\">
	<head>
		<title>Post #1: The First One</title>
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
		<title>Post #2: The Second One</title>
	</head>
	<body>
		<h2>Post #2: The Second One</h2>

		<p>
			
\"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\"

		</p>
	</body>
</html>
";
    assert!(contents == expected);

    Ok(())
}

#[test]
fn test_paths_changed() -> Result<(), Box<dyn std::error::Error>> {
    let root = env::current_dir()?.join("tests/paths-changed");
    // Build site
    assert!(Command::new(get_crate_exe().unwrap())
            .current_dir(&root)
            .status().is_ok());
    // Check static files
    assert!(root.join("target/img/emoticon.png").is_file());
    assert!(root.join("target/style.css").is_file());
    // Check pages
    let mut file = fs::File::open(&root.join("target/home.html"))?;
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

    let mut file = fs::File::open(&root.join("target/post-1.html"))?;
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

    let mut file = fs::File::open(&root.join("target/post-2.html"))?;
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

    let mut file = fs::File::open(&root.join("target/github.html"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let expected = "<!DOCTYPE html><html>
	<head><meta http-equiv='refresh' content='0; url=\"https://github.com/pennbauman/terafirma\"'/></head>
	<body><p><a href='https://github.com/pennbauman/terafirma'>https://github.com/pennbauman/terafirma</a></p></body>
</html>";
    assert!(contents == expected);

    Ok(())
}
