use actix_web::{HttpResponse, HttpRequest, web};
use askama::Template;
use crate::models::ui::{HomeTemplate, LoansTemplate, ProfileTemplate};
use crate::models::app_state::AppState;
use crate::db_operations::session::check_session;

// Get Controllers
pub async fn dashboard_get(
  app_state: web::Data<AppState>,
  req: HttpRequest,
  path: web::Path<String>) -> HttpResponse {
    let mut conn = app_state.redis_pool.get().await.unwrap();
    if check_session(&mut conn, &req).await {
        println!("Session exists\n");
        let page = path.into_inner();
        let template = match page.as_str() {
            "profile" => ProfileTemplate{}.render(),
            "loans" => LoansTemplate{}.render(),
            _ => HomeTemplate{}.render(),
        };
        HttpResponse::Ok().content_type("text/html").body(template.unwrap())
    }else{
        println!("Session doesnt exists\n");
        HttpResponse::Found()
        .insert_header(("LOCATION", "/auth/login"))
        .finish()
    }
    
}