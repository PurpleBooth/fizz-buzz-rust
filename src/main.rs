extern crate iron;
extern crate router;

use std::env;

use iron::prelude::*;
use iron::status;
use router::Router;

mod fizz_buzz;

fn main() {
    let port: String = env::var("PORT").unwrap_or("8080".to_string());

    let router = configure_router();
    let listen_on = format!("0.0.0.0:{}", port);
    let _server = Iron::new(router).http(listen_on.to_string()).unwrap();

    println!("On {}", listen_on);
}

fn configure_router() -> Router {
    let mut router = Router::new();
    router.get("/", index_action, "index");
    router.get("/:value", fizz_buzz_action, "fizz_buzz");

    return router;
}

fn index_action(_req: &mut Request) -> IronResult<Response> {
    return Ok(Response::with((status::Ok, "OK")));
}

fn fizz_buzz_action(req: &mut Request) -> IronResult<Response> {
    let ref value = req
        .extensions
        .get::<Router>()
        .unwrap()
        .find("value")
        .unwrap_or("/");

    Ok(Response::with((
        status::Ok,
        fizz_buzz::fizz_buzz(value.parse::<i128>().unwrap_or(1)),
    )))
}

#[cfg(test)]
mod tests {
    use iron::{status, Headers};
    use iron_test::request;
    use iron_test::response;

    use crate::configure_router;

    #[test]
    fn test_index_returns_ok() {
        let router = configure_router();
        let response = request::get("http://localhost:8080/", Headers::new(), &router).unwrap();
        let result_code = response.status;
        let result_body = response::extract_body_to_string(response);

        assert_eq!("OK".to_string(), result_body);
        assert_eq!(status::Ok, result_code.unwrap());
    }

    #[test]
    fn test_fizz_endpoint_returns_fizz_and_an_ok_status() {
        let router = configure_router();
        let response = request::get("http://localhost:8080/3", Headers::new(), &router).unwrap();
        let result_code = response.status;
        let result_body = response::extract_body_to_string(response);

        assert_eq!("fizz".to_string(), result_body);
        assert_eq!(status::Ok, result_code.unwrap());
    }

    #[test]
    fn test_fizz_endpoint_returns_ok_status_even_for_weird_values() {
        let router = configure_router();
        let response = request::get(
            "http://localhost:8080/asfergwegwe-1",
            Headers::new(),
            &router,
        )
        .unwrap();
        let result_code = response.status;
        let result_body = response::extract_body_to_string(response);

        assert_eq!("1".to_string(), result_body);
        assert_eq!(status::Ok, result_code.unwrap());
    }
}
