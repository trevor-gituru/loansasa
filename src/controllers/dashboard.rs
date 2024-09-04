use actix_web::{HttpResponse, HttpRequest, web};
use askama::Template;
use crate::models::ui::DashBoardTemplate;
use crate::models::app_state::AppState;
use crate::db_operations::session::check_session;

// Get Controllers
pub async fn dashboard_get(
  app_state: web::Data<AppState>,
  req: HttpRequest) -> HttpResponse {
    let mut conn = app_state.redis_pool.get().await.unwrap();
    if check_session(&mut conn, &req).await {
        println!("Session exists\n");
        let template = DashBoardTemplate{};
        HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
    }else{
        println!("Session doesnt exists\n");
        HttpResponse::Found()
        .insert_header(("LOCATION", "/auth/login"))
        .finish()
    }
    
}