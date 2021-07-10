use actix_web::{HttpResponse, Responder};

#[actix_web::get("/")]
pub async fn get_index() -> impl Responder {
    HttpResponse::PermanentRedirect()
        .header(
            "Location",
            "https://github.com/SlashNephy/annict-profile-card",
        )
        .finish()
}
