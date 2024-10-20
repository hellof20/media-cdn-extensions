use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { Box::new(MyHttpContext) });
}}

struct MyHttpContext;

impl Context for MyHttpContext {}

impl HttpContext for MyHttpContext {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        // 添加 pp-test header
        self.add_http_request_header("pp-test", "test");
        info!("Added pp-test header with value 'test'");

        if let Some(user_agent) = self.get_http_request_header("user-agent") {
            info!("Received User-Agent: {}", user_agent);
            
            let client_type = determine_client_type(&user_agent);
            
            info!("Determined client type: {}", client_type);
            self.add_http_request_header("client", client_type);
        } else {
            info!("No User-Agent header found");
            self.add_http_request_header("client", "unknown");
        }

        Action::Continue
    }
}

fn determine_client_type(user_agent: &str) -> &'static str {
    let user_agent_lower = user_agent.to_lowercase();
    
    let client_type = if user_agent_lower.contains("android") {
        "android"
    } else if user_agent_lower.contains("iphone") || user_agent_lower.contains("ipad") || user_agent_lower.contains("ipod") {
        "ios"
    } else if is_mobile_user_agent(&user_agent_lower) {
        "mobile"
    } else {
        "pc"
    };

    info!("User-Agent '{}' determined as '{}'", user_agent, client_type);
    client_type
}

fn is_mobile_user_agent(user_agent: &str) -> bool {
    let mobile_keywords = [
        "webos", "blackberry", "windows phone", "opera mini", "opera mobi", "nokia", "symbian"
    ];
    
    let is_mobile = mobile_keywords.iter().any(|&keyword| user_agent.contains(keyword));
    info!("is_mobile_user_agent check result: {}", is_mobile);
    is_mobile
}