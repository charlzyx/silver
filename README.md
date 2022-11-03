# silver

```rust
#[tokio::main]
async fn main() {
    let path = std::env::args().nth(1).unwrap_or(".".into());
    let port = std::env::args()
        .nth(2)
        .unwrap_or("23333".into())
        .parse::<u16>()
        .unwrap_or(23333_u16);
    println!(
        "Static File Http Server Ver 0.0.1\nRoot Dir: {}\nUsage: httpSvr [root_dir_default_.] [port_default_23333]\nhttp://127.0.0.1:{}\n",
        path,port
    );
    let api = warp::fs::dir(path);
    let server = warp::serve(api);
    server.run(([0, 0, 0, 0], port)).await;
}
```

# usage

silver [root=.] [port=23333]
