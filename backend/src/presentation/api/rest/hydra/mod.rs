pub mod consent;
pub mod error;
pub mod kratos_login;
pub mod login;
pub mod logout;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(login::login_get)
        .service(login::login_post)
        .service(consent::consent_get)
        .service(consent::consent_post)
        .service(logout::logout)
        .service(kratos_login::login_kratos)
        .service(error::hydra_error);
}
