# silver

## Overview
Super simple and smart static server as a single executable, build for [tauri app sidecar](https://tauri.app/zh/v1/guides/building/sidecar/).

## Features
-  `try_files` automic, useful for web app history router.
- reverse proxy by headers `silverproxy`, look at bottom.


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
## Advance:: proxy

with speical header `silverproxy` with proxy to taget `host:port`,
**no* `http/https` prefix and last `/` but `port` is required

`silverhost` is not required, if not setting, will be same with `silverproxy`

example

```sh
curl --location --request GET 'http://127.0.0.1:2333/api/v5/users/charlzyx/repos' \
--header 'silverproxy: gitee.com:80' \
--header 'silverhost: gitee.com' \
```

# Powered by
- [rouille](https://github.com/tomaka/rouille)
- [tiny-http](https://github.com/tiny-http/tiny-http)