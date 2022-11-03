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
