use crate::application::commands::CommandHandler;
use crate::application::commands::auth::login::LoginCommand;
use crate::domain::ports::inbound::login::LoginCredentials;
use crate::infrastructure::adapters::http::cookies::RequestResponseCookies;
use crate::infrastructure::di::container::UseCases;
use crate::presentation::api::rest::v1::dto::auth::LoginDto;
use crate::presentation::api::rest::v1::handlers::utils::extract_cookie;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use std::sync::Arc;

pub async fn login(
    req: HttpRequest,
    use_cases: web::Data<Arc<UseCases>>,
    dto: web::Json<LoginDto>,
) -> HttpResponse {
    let credentials: LoginCredentials = match dto.into_inner().try_into() {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };

    let command = LoginCommand {
        credentials,
        cookie: extract_cookie(&req),
    };

    match use_cases.commands.login.handle(command).await {
        Ok(session_token) => {
            req.extensions_mut()
                .get_mut::<RequestResponseCookies>()
                .unwrap()
                .add(session_token);

            HttpResponse::Ok().finish()
        }
        Err(e) => HttpResponse::Unauthorized().body(e.to_string()),
    }
}
