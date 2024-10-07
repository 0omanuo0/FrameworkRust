
mod server;
use server::Server;
use server::Request;


fn home(_request: Request) -> String {
    return String::from("<h1>HOLAAA</h1>");
}

fn example_with_params(request: Request) -> String {
    let mut response = String::from("<h1>Params:</h1>");
    if request.session.get("user").is_some() {
        response.push_str(&format!("<p>User: {}</p>", request.session.get("user").unwrap()));
    }

    for param in request.params {
        response.push_str(&format!("<p>{}</p>", param));
    }
    return response;
}

fn main() {
    // let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let mut server = Server::new("127.0.0.1:8080");
    server.add_route("/", home);
    server.add_route("/example/:param1/other/:param2", example_with_params);

    server.start_listener();
}