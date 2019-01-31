#![deny(warnings)]
#[macro_use]
extern crate warp;

use warp::Filter;

fn main() {
    let route_home = warp::filters::path::end()
            .map(|| "home");

    let route_info = warp::path("info")
        .map(|| "info");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_method("post")
        .allow_method("get")
        .allow_header("content-type");

    let route_post = warp::path("post")
        .and(warp::post2())
        .map(|| "endpoint")
        .with(cors.clone())
        .boxed();

    let api_a = path!("api" / "a")
        .map(|| "a");
    
    let api_b = path!("api" / "b")
        .and(warp::post2())
        .map(|| "b")
        .with(cors)
        .boxed();

    let api = api_a.or(api_b);


    let routes = route_home
        .or(api)
        .or(route_info)
        .or(route_post);

    warp::serve(routes).run(([127, 0, 0, 1], 3030));
}

