# silver

## Overview
Super simple static server as a single executable, build for [tauri app sidecar](https://tauri.app/zh/v1/guides/building/sidecar/).

## Features
- Built with [Tokio](https://github.com/tokio-rs/tokio) and [warp](https://github.com/seanmonstar/warp)


## SourceCode

**👇That's all.**

```rust
use std::env;

#[tokio::main]
async fn main() {
    let path = env::args().nth(1).unwrap_or(
        env::current_dir()
            .unwrap()
            .to_str()
            .unwrap_or(".")
            .to_string(),
    );

    let port = env::args()
        .nth(2)
        .unwrap_or("2333".into())
        .parse::<u16>()
        .unwrap_or(2333_u16);

    println!(
        "silver:: a static files server ver {}\n www root: {}\nUsage: silver [root_dir=$PWD] [port=2333]\nhttp://0.0.0.0:{}\n",
        env!("CARGO_PKG_VERSION"), path,port
    );

    let api = warp::fs::dir(path);

    let server = warp::serve(api);

    server.run(([0, 0, 0, 0], port)).await;
}

```

## Usage

Download silver_xxx.zip for your platform. [Releases](https://github.com/charlzyx/silver/releases).

### Usage executable binary file

```sh
silver [root=$PWD] [port=2333]
```



### Usage in tauri app
1. download all of platform zips you needs, and unzip all of them to `src-tauri/binaries/` folder.
2. `tauri.config.json` looks like this.

```json
{
  "tauri": {
    "bundle": {
      "externalBin": [
        "binaries/silver"
      ]
    },
    "allowlist": {
      "shell": {
        "sidecar": true,
        "scope": [
          { "name": "binaries/silver", "sidecar": true }
        ]
      }
    }
  }
}
```

3. Example use in rust with args. [and more offical docs](https://tauri.app/zh/v1/guides/building/sidecar/)

```rust
use tauri::api::path;
use tauri::api::process::{Command, CommandEvent};

pub fn serve() {
    let port = 8686;

    let root = String::from(
        path::home_dir()
            .unwrap()
            .join("www")
            .to_str()
            .unwrap(),
    );


    let (mut rx, mut child) = Command::new_sidecar("silver")
        .expect("failed to create `silver` binary command")
        .args([root, port.to_string()])
        .spawn()
        .expect("Failed to spawn sidecar");


    tauri::async_runtime::spawn(async move {
        // read events such as stdout
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line) = event {
                println!("{}", line);
                // window
                //     .emit("message", Some(format!("'{}'", line)))
                //     .expect("failed to emit event");
                // // write to stdin
                child.write("message from Rust\n".as_bytes()).unwrap();
            }
        }
    });
}
```
