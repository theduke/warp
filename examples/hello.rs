#![deny(warnings)]
extern crate warp;

use warp::Filter;

fn main() {
    // Match any request and return hello world!
    /*
    let routes = warp::any()
        .map(|| "Hello, World!");
        */
    let plain = warp::path("plaintext")
        .map(|| "Hello, World!");
    let json = warp::path("json")
        .map(|| warp::reply::json(&["Hello, World"]));
    let routes = plain.or(json);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030));
}

