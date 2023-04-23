use std::path::PathBuf;

fn get_home_dir() -> PathBuf {
    let home_dir = match home::home_dir() {
        Some(path) => path,
        None => {
            panic!("Could not find home directory");
        }
    };

    home_dir
}

pub fn get_www_dir() -> PathBuf {
    let home_dir = get_home_dir();
    let www_dir = home_dir.join(".www");

    www_dir
}

pub const ERR_HTML: &str = "
<html>
<head>
    <title>Error</title>
</head>
<body>
    <p>{}</p>
</body>
";

pub const DIR_HTML: &str = "
<html>
<head>
    <title>Index of {:1}</title>
</head>
<body>
    <h1>Index of {:1}</h1>
    <ul>
        {:2}
    </ul>
</body>
";
