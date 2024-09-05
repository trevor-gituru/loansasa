use actix_web::HttpRequest;

pub fn get_ip(req: &HttpRequest) -> Option<String> {
    // Try to get IP from the X-Forwarded-For header from reverse proxy
    if let Some(forwarded_for) = req.headers().get("X-Forwarded-For") {
        if let Ok(ip) = forwarded_for.to_str() {
            return Some(ip.split(',').next()?.trim().to_string());
        }
    }

    // Fallback to the remote IP from the connection info for direct connection
    req.connection_info()
        .realip_remote_addr()
        .map(|ip| ip.to_string())
}

pub fn get_browser(req: &HttpRequest) -> Option<String> {
    // Try to get the User-Agent header
    if let Some(user_agent) = req.headers().get("User-Agent") {
        if let Ok(browser_info) = user_agent.to_str() {
            return Some(browser_info.to_string());
        }
    }

    // Fallback to None if the User-Agent header is not present
    None
}
