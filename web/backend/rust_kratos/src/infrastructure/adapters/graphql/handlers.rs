use crate::infrastructure::adapters::graphql::schema::AppSchema;
use crate::infrastructure::adapters::http::cookies::ResponseCookies;
use actix_web::{HttpRequest, HttpResponse, Result, web};
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::GraphQLRequest;

#[derive(Clone, Debug)]
pub struct UserIp(pub String);

fn get_real_ip(req: &HttpRequest) -> UserIp {
    if let Some(xff) = req.headers().get("x-forwarded-for") {
        if let Ok(val) = xff.to_str() {
            if let Some(ip) = val.split(',').next() {
                return UserIp(ip.trim().to_string());
            }
        }
    }
    UserIp(
        req.connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string(),
    )
}

pub async fn graphql_handler(
    schema: web::Data<AppSchema>,
    req: GraphQLRequest,
    http_req: HttpRequest,
) -> Result<HttpResponse> {
    let response_cookies = ResponseCookies::new();
    let cookie_header = http_req
        .headers()
        .get(actix_web::http::header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string());

    let mut request = req.into_inner();
    request = request.data(cookie_header);
    request = request.data(response_cookies.clone());
    request = request.data(get_real_ip(&http_req));

    let response = schema.execute(request).await;
    let cookies = response_cookies.get_cookies().await;
    let mut http_response = HttpResponse::Ok();
    for cookie in cookies {
        http_response.insert_header(("Set-Cookie", cookie));
    }
    Ok(http_response.json(response))
}

pub async fn graphql_playground() -> Result<HttpResponse> {
    let html = GraphiQLSource::build().endpoint("/graphql").finish();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
