pub mod current_user;
pub mod login;
pub mod logout;
pub mod recovery;
pub mod register;
pub mod settings;
pub mod utils;
pub mod verification;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register::register))
            .route("/login", web::post().to(login::login))
            .route("/logout", web::get().to(logout::logout))
            .route("/me", web::get().to(current_user::current_user))
            .route("/recovery", web::post().to(recovery::recovery))
            .route("/settings", web::put().to(settings::update_settings))
            .route("/verify/link", web::post().to(verification::verify_by_link))
            .route(
                "/verify/code/send",
                web::post().to(verification::send_verification_code),
            )
            .route(
                "/verify/code/submit",
                web::post().to(verification::submit_verification_code),
            ),
    );
}
