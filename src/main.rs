use ascii::AsciiString;
use std::fs;
use std::path::Path;

use local_ip_address::local_ip;
use tiny_http;

mod util;

fn main() {
    let (root, addr) = util::ready();

    let server = tiny_http::Server::http(addr).unwrap();

    let port = server.server_addr().to_ip().unwrap().port();
    let my_local_ip = local_ip().unwrap();

    println!(
        r#"
------------------------------------------------------------------------
silver:: a static files server ver {} ,the www root is: {}
now listening...
http://127.0.0.1:{}
http://{:?}{}
------------------------------------------------------------------------
    "#,
        env!("CARGO_PKG_VERSION"),
        root,
        port,
        my_local_ip,
        port
    );

    loop {
        let rq = match server.recv() {
            Ok(rq) => rq,
            Err(e) => {
                println!("some things error: {} \n --said by silver.", e);
                break;
            }
        };

        let url = rq.url().to_string();
        let maybe = util::try_files(&root, util::raw(&url));

        if maybe.is_err() {
            let message = format!("some things error when read file {}\n", maybe.err().unwrap());
            let resp = tiny_http::Response::from_string(message + " -- said by silver.")
                .with_status_code(404);
            let _ = rq.respond(resp);
        } else {
            let file_name = maybe.unwrap();
            let path = Path::new(file_name.to_str().unwrap());
            // println!("to open file is {}", &file_name.to_str().unwrap());
            let file = fs::File::open(&file_name);

            let content_type = util::get_content_type(path);

            if file.is_ok() {
                let response = tiny_http::Response::from_file(file.unwrap());

                let response = response.with_header(tiny_http::Header {
                    field: "Content-Type".parse().unwrap(),
                    value: AsciiString::from_ascii(content_type).unwrap(),
                });

                let _ = rq.respond(response);
            } else {
                let resp = tiny_http::Response::from_string("Not Found. -- said by silver.")
                    .with_status_code(404);
                let _ = rq.respond(resp);
            }
        }
    }
}
