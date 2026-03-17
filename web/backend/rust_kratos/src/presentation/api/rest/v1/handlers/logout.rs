use crate::application::commands::CommandHandler;
use crate::application::commands::auth::logout::LogoutCommand;
use crate::infrastructure::di::container::UseCases;
use crate::presentation::api::rest::v1::handlers::utils::extract_cookie;
use actix_web::{HttpRequest, HttpResponse, web};
use std::sync::Arc;

pub async fn logout(req: HttpRequest, use_cases: web::Data<Arc<UseCases>>) -> HttpResponse {
    let command = LogoutCommand {
        cookie: extract_cookie(&req),
    };

    match use_cases.commands.logout.handle(command).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
