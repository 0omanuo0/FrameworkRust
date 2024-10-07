pub fn match_route(route: &String, request: &String, params: &mut Vec<String>) -> bool {
    let route_parts = route.split("/").collect::<Vec<&str>>();
    let request_parts = request.split("/").collect::<Vec<&str>>();

    // Si las longitudes no coinciden, no puede ser una coincidencia
    if route_parts.len() != request_parts.len() {
        return false;
    }

    for i in 0..route_parts.len() {
        let route_part = route_parts[i];
        let request_part = request_parts[i];

        if route_part.starts_with(":") {
            params.push(request_part.to_string()); 
        } else if route_part != request_part {
            return false;
        }
    }

    true // Si llega hasta aquí, la ruta coincide
}
