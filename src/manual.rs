use hyper::{Body, Request, Response, StatusCode};

pub async fn manual_server(req: Request<Body>) -> Result<Response<Body>, hyper::error::Error> {
    let mut response = Response::builder().header("Content-Type", "text/html").body(Body::empty()).expect("");

    match req.uri().path() {
        "/manual" => {
            *response.body_mut() = html_manual_front();
        },
        "/manual/foo" => {
            *response.body_mut() = req.into_body();
        },
        "/manual/bar" => {
            *response.body_mut() = req.into_body();
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok(response)
}

fn html_page(body: &str) -> String {
    format!("<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><title>manual</title></head><body>{}</body></html>", body)
}

fn html_manual_front() -> Body {
    let entries: Vec<&str> = vec!["foo", "bar"];
    let mut output = "".to_string();

    entries.iter().for_each(|entry| {
        let link = format!("\"http://localhost:12345/manual/{}\"", entry);
        output.push_str(format!("<p><a href={}>{}</a></p>", link, entry).as_str())
    });
        
    Body::from(html_page(output.as_str()))
}