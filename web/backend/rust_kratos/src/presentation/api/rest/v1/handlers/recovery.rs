use crate::application::commands::CommandHandler;
use crate::application::commands::account::recovery::RecoveryCommand;
use crate::domain::ports::inbound::recovery::RecoveryRequest;
use crate::infrastructure::di::container::UseCases;
use crate::presentation::api::rest::v1::dto::auth::RecoveryDto;
use crate::presentation::api::rest::v1::handlers::utils::extract_cookie;
use actix_web::{HttpRequest, HttpResponse, web};
use std::sync::Arc;

pub async fn recovery(
    req: HttpRequest,
    use_cases: web::Data<Arc<UseCases>>,
    dto: web::Json<RecoveryDto>,
) -> HttpResponse {
    let request: RecoveryRequest = match dto.into_inner().try_into() {
        Ok(r) => r,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };

    let command = RecoveryCommand {
        request,
        cookie: extract_cookie(&req),
    };

    match use_cases.commands.recovery.handle(command).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
