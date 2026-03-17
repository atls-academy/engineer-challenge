use crate::application::commands::CommandHandler;
use crate::application::commands::account::verification::{
    SendVerificationCodeCommand, SubmitVerificationCodeCommand, VerifyByLinkCommand,
};
use crate::domain::ports::inbound::verification::{SendCodeRequest, VerifyByLinkRequest};
use crate::infrastructure::di::container::UseCases;
use crate::presentation::api::rest::v1::dto::auth::{
    SendVerificationCodeDto, SubmitVerificationCodeDto, VerifyByLinkDto,
};
use crate::presentation::api::rest::v1::handlers::utils::extract_cookie;
use actix_web::{HttpRequest, HttpResponse, web};
use std::sync::Arc;

pub async fn verify_by_link(
    req: HttpRequest,
    use_cases: web::Data<Arc<UseCases>>,
    dto: web::Json<VerifyByLinkDto>,
) -> HttpResponse {
    let request: VerifyByLinkRequest = match dto.into_inner().try_into() {
        Ok(r) => r,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };

    let command = VerifyByLinkCommand {
        request,
        cookie: extract_cookie(&req),
    };

    match use_cases.commands.verification.handle(command).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn send_verification_code(
    req: HttpRequest,
    use_cases: web::Data<Arc<UseCases>>,
    dto: web::Json<SendVerificationCodeDto>,
) -> HttpResponse {
    let request: SendCodeRequest = match dto.into_inner().try_into() {
        Ok(r) => r,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };

    let command = SendVerificationCodeCommand {
        request,
        cookie: extract_cookie(&req),
    };

    match use_cases.commands.verification.handle(command).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn submit_verification_code(
    req: HttpRequest,
    use_cases: web::Data<Arc<UseCases>>,
    dto: web::Json<SubmitVerificationCodeDto>,
) -> HttpResponse {
    let cookie = match extract_cookie(&req) {
        Some(c) => c,
        None => {
            return HttpResponse::BadRequest()
                .body("Cookie is required to submit verification code");
        }
    };

    let command = SubmitVerificationCodeCommand {
        request: dto.into_inner().into(),
        cookie,
    };

    match use_cases.commands.verification.handle(command).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
