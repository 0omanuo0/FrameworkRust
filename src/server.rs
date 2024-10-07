
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::collections::HashMap;

// import ./request.rs
mod headers;
mod session;
mod request;
mod public_globals;
mod url_functions;
pub use request::Request;
use headers::Headers;
use url_functions::*;
use public_globals::*;





pub struct Server {
    listener: TcpListener,
    // routes: path, partial_route (until:), has_params, handler
    routes: Vec<(String, bool, fn(Request) -> String)>,
    // sessions: HashMap<session_id, session>
    sessions: HashMap<String, session::Session>,
}


impl Server {
    pub fn new(addr: &str) -> Server {
        Server {
            listener: TcpListener::bind(addr).unwrap(),
            routes: Vec::new(),
            sessions: HashMap::<String, session::Session>::new(),
        }
    }

    // handle client function
    fn handler(&self, mut stream: TcpStream) {
        let mut buffer = [0; 65536];
        stream.read(&mut buffer).unwrap();
        let request = String::from_utf8_lossy(&buffer[..]);
        let response_base = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n";

        let headers = Headers::new(request.to_string());
        // println!("Request: {}", request);

        // Aquí definimos un vector mutable para pasar parámetros
        let mut params = Vec::new();

        let mut session = session::Session::new();
        if headers.cookie.contains_key(COOKIE_NAME) {
            // find session
            let session_id = headers.cookie["session_id"].clone();
            if let Some(s) = self.sessions.get(&session_id) {
                session = s.copy();
            }
        }


        let mut fn_response:String = String::new();

        for route in &self.routes {
            // type of route is (path, has_params, handler)
            // Comparar primero la ruta exacta sin parámetros
            if headers.route == route.0 {
                let response = (route.2)(
                        Request::new_with_params(params.clone(), headers, session)
                    ); // Clonar headers
                fn_response = response;

                break;
            }
            else if match_route(&route.0, &headers.route, &mut params) {
                let response = (route.2)(
                        Request::new_with_params(params.clone(), headers, session)
                    );
                fn_response = response;

                break;
            }
        }

        if fn_response == ""{
            fn_response = String::from("<h1>404 Not Found</h1>");
        }

        let content_length = fn_response.len();
        let response = format!(
            "{}Content-Length: {}\r\n\r\n{}",
            response_base,
            content_length,
            fn_response
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }


    pub fn add_route(&mut self, path: &str, handler: fn(Request) -> String) {
        // params are defined as: /path/:param1/:param2
        if path.contains(":") {
            self.routes.push((path.to_string(),  true, handler));
            return;
        }
        self.routes.push((path.to_string(),  false, handler));
    }

    pub fn start_listener(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handler(stream);
                }
                Err(e) => {
                    eprintln!("failed: {}", e);
                }
            }
        }
    }
}