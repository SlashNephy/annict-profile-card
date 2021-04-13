use actix_web::{Responder, HttpResponse};

#[actix_web::get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::PermanentRedirect()
        .header("Location", "https://github.com/SlashNephy/annict-profile-card")
        .finish()
}
