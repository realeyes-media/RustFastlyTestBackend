use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::Read;
use fastly::http::{header, Method, StatusCode};
use fastly::{Error, Request, Response};

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Only permit GET requests.
    if req.get_method() != Method::GET {
        return Ok(Response::from_body("Method not allowed")
            .with_status(StatusCode::METHOD_NOT_ALLOWED)
            .with_header(
                header::ALLOW,
                format!("{}, {}", Method::GET, Method::OPTIONS),
            ));
    }

    // just serve the same HLS file for now, can do paths later
    let bytes: &[u8] = include_bytes!("static/master.m3u8");
    Ok(Response::from_body(bytes))
}
