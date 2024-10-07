use std::collections::HashMap;
use super::public_globals::{self, COOKIE_NAME};

pub struct Headers {
    pub method: String,
    pub route: String,
    pub query: String,
    pub version: String,

    pub cookie: HashMap<String, String>,

    pub headers: HashMap<String, String>
}

fn get_route_parts(route: String) -> (String, String, String) {
    let mut path = String::new();
    let mut query = String::new();
    let mut fragment = String::new();

    let mut parts = route.splitn(2, '#');
    if let Some(first_part) = parts.next() {
        if let Some(second_part) = parts.next() {
            fragment = second_part.to_string();
        }

        let mut path_query = first_part.splitn(2, '?');
        if let Some(path_part) = path_query.next() {
            path = path_part.to_string();
        }
        if let Some(query_part) = path_query.next() {
            query = query_part.to_string();
        }
    }

    (path, query, fragment)
}

fn getSessionCookie(cookies: String) -> HashMap<String, String> {
    let mut parts = cookies.split(";");

    let mut cookies = HashMap::new();
    for part in parts {
        let mut parts = part.split("=");
        let key = parts.next().unwrap().to_string();
        let value = parts.next().unwrap().to_string();
        cookies.insert(key, value);
    }

    return cookies;
}


fn parse_headers(req_str: String) -> (String, String, String, String, HashMap<String, String>) {
    let mut lines = req_str.lines();
    let line = lines.next();
    println!("\nLINE: {:?}", line);

    let mut parts = line.unwrap().split_whitespace();
    let method = parts.next().unwrap().to_string();


    // get each part of the route as: /path?query#fragment
    let route = parts.next().unwrap().to_string();
    let (route, query, _) = get_route_parts(route);    

    let version = parts.next().unwrap().to_string();

    // Parse the headers
    let mut headers = HashMap::new();
    for line in lines {
        if line == "" {
            break;
        }
        let mut parts = line.split(": ");
        let key = parts.next().unwrap().to_string();
        let value = parts.next().unwrap().to_string();
        headers.insert(key, value);
    }

    return (method, route, query, version, headers);
}

impl Headers {
    pub fn new(req_str:String) -> Headers {
        let (method, route, query, version, headers) = parse_headers(req_str);
        
        let cookies = getSessionCookie(headers.get("Cookie").unwrap().to_string());

        Headers {
            method: method,
            route: route,
            query: query,
            version: version,
            cookie: cookies,
            headers: headers,
        }
    }
    
}