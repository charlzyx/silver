use mime_guess;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub fn raw(url: &str) -> &str {
    if let Some(pos) = url.bytes().position(|c| c == b'?') {
        url.split_at(pos).0
    } else {
        url
    }
}

pub fn findup(mut p: PathBuf, root: PathBuf) -> PathBuf {
    if p == root {
        p.push("index.html");
        p
    } else if p.is_dir() {
        p.push("index.html");
        if p.exists() {
            p
        } else {
            // remove index.html
            p.pop();
            // to parent
            p.pop();

            findup(p, root)
        }
    } else {
        if p.exists() {
            p
        } else {
            p.pop();

            findup(p, root)
        }
    }
}

pub fn try_files(base: &str, url: &str) -> Result<PathBuf, std::io::Error> {
    let root = Path::new(base).to_owned().to_path_buf();
    let target = {
        #[allow(clippy::redundant_clone)]
        let mut path = root.to_path_buf();
        for component in url.split('/') {
            path.push(component);
        }
        path
    };
    // println!("-- target {}, url {}", target.to_str().unwrap(), url);
    Ok(findup(target.clone(), root))
}

pub fn ready() -> (String, SocketAddr) {
    let root = env::args().nth(1).unwrap_or(
        env::current_dir()
            .unwrap()
            .to_str()
            .unwrap_or(".")
            .to_string(),
    );

    let abs = fs::canonicalize(PathBuf::from(&root)).unwrap();
    let abs_root = format!("{}", abs.to_str().unwrap());

    let host = "127.0.0.1";

    let port = env::args()
        .nth(2)
        .unwrap_or("2333".into())
        .parse::<u16>()
        .unwrap_or(2333_u16);

    let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

    (format!("{}", &abs_root), addr)
}

pub fn get_content_type(path: &Path) -> String {
    let mime = mime_guess::from_path(&path).first_or_octet_stream();
    mime.to_string()
}
