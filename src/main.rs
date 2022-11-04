#![allow(unreachable_code)]
#[macro_use]
extern crate rouille;

use std::fs;
use std::path::Path;

use local_ip_address::local_ip;
use mime_guess;
use rouille::proxy;
use rouille::{Request, Response};

mod silver;

fn main() {
    let (root, addr, port) = silver::parse();
    let my_local_ip = local_ip().unwrap();

    println!(
        r#"
--------------------------------------------------
silver:: a static files server ver {}
www root is: {}
now listening...
http://127.0.0.1:{}
http://{:?}:{}
--------------------------------------------------
  "#,
        env!("CARGO_PKG_VERSION"),
        root,
        port,
        my_local_ip,
        port
    );

    fn handle_proxy(request: &Request) -> Response {
        let host = request.header("silverhost");

        let config = match request.header("silverproxy") {
            Some(h) => proxy::ProxyConfig {
                addr: h,
                replace_host: Some(format!("{}", host.unwrap_or(h)).into()),
            },
            _ => return Response::empty_404(),
        };

        // println!("addr {}", &config.addr);
        // println!("host {}", &config.replace_host.clone().unwrap().to_string());

        proxy::full_proxy(request, config).unwrap()
    }
    // The `start_server` starts listening forever on the given address.
    rouille::start_server(&addr, move |request| {
        if request.header("silverproxy").is_some() {
            // println!("header proxy {} ", request.header("proxy").unwrap());
            return handle_proxy(request);
        }
        let trytry = silver::try_files(&root, &request.url());
        if trytry.is_err() {
            let message = format!(
                "some things error when read file {}\n",
                trytry.err().unwrap()
            );

            Response::text(format!("somthings error {} \n -- said by silver.", message))
                .with_status_code(502)
        } else {
            let file_name = trytry.unwrap();
            let path = Path::new(file_name.to_str().unwrap());

            let file = match fs::File::open(&file_name) {
                Ok(f) => f,
                Err(_) => {
                    return Response::text("Not Found. -- said by silver.").with_status_code(404)
                }
            };

            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            let content_type = mime.to_string();

            Response::from_file(content_type, file)
        }
    });
}
