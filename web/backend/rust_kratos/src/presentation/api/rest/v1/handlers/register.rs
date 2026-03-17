use crate::application::commands::CommandHandler;
use crate::application::commands::identity::register::RegisterCommand;
use crate::domain::ports::inbound::registration::RegistrationData;
use crate::infrastructure::adapters::http::cookies::RequestResponseCookies;
use crate::infrastructure::di::container::UseCases;
use crate::presentation::api::rest::v1::dto::auth::RegisterDto;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use std::sync::Arc;

pub async fn register(
    req: HttpRequest,
    use_cases: web::Data<Arc<UseCases>>,
    dto: web::Json<RegisterDto>,
) -> HttpResponse {
    let data: RegistrationData = match dto.into_inner().try_into() {
        Ok(d) => d,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };

    match use_cases
        .commands
        .register
        .handle(RegisterCommand { data })
        .await
    {
        Ok(result) => {
            req.extensions_mut()
                .get_mut::<RequestResponseCookies>()
                .unwrap()
                .add(result.session_cookie);

            HttpResponse::Created().finish()
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
