use crate::infrastructure::adapters::hydra::client::HydraClient;
use crate::infrastructure::adapters::hydra::requests::accept_login;
use actix_web::{HttpRequest, HttpResponse, Responder, post, web};
use serde::Deserialize;
use std::sync::Arc;
use tracing::instrument;

#[derive(Deserialize, Debug)]
struct KratosLoginRequest {
    login_challenge: String,
}

#[post("/login/kratos")]
#[instrument(skip(hydra, req))]
pub async fn login_kratos(
    body: web::Json<KratosLoginRequest>,
    hydra: web::Data<Arc<HydraClient>>,
    req: HttpRequest,
) -> impl Responder {
    let cookie = req
        .headers()
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let client = reqwest::Client::new();
    let whoami = match client
        .get("http://localhost:4433/sessions/whoami")
        .header("Cookie", &cookie)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({"error": e.to_string()}));
        }
    };

    if !whoami.status().is_success() {
        return HttpResponse::Unauthorized()
            .json(serde_json::json!({"error": "no active kratos session"}));
    }

    let session: serde_json::Value = match whoami.json().await {
        Ok(s) => s,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()}));
        }
    };

    let subject = match session["identity"]["id"].as_str() {
        Some(id) => id.to_string(),
        None => {
            return HttpResponse::Unauthorized()
                .json(serde_json::json!({"error": "identity id not found"}));
        }
    };

    let accepted = match accept_login(&hydra, &body.login_challenge, subject, 3600).await {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()}));
        }
    };

    HttpResponse::Ok().json(serde_json::json!({
        "redirect_to": accepted.redirect_to
    }))
}
