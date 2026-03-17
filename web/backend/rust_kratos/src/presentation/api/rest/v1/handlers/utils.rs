use actix_web::HttpRequest;

pub fn extract_cookie(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

pub fn extract_ip(req: &HttpRequest) -> &str {
    req.headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
}
