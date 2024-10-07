
use super::headers::Headers;
use super::session::Session;



pub struct Request {
    pub params: Vec<String>,

    pub method: String,
    pub route: String,
    pub query: String,
    pub headers: Headers, 

    pub content: String,

    pub session: Session,
}

impl Request{
    pub fn new() -> Request {
        Request {
            params: Vec::new(),
            method: String::new(),
            route: String::new(),
            query: String::new(),
            headers: Headers::new("".to_string()),
            content: String::new(),
            session: Session::new(),
        }
    }
    pub fn new_with_params(params_n: Vec<String>, headers_n: Headers, session_n: Session) -> Request {
        Request {
            params: params_n,
            method: headers_n.method.clone(),
            route: headers_n.route.clone(),
            query: headers_n.query.clone(),
            headers: headers_n,
            content: String::new(),
            session: session_n,
        }
    }

}
