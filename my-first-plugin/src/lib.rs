use proxy_wasm::{
    traits::{Context, HttpContext, RootContext},
    types::{Action, ContextType, LogLevel},
};

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Debug);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(MyFirstPluginRoot {
        })
    });
}}

struct MyFirstPluginRoot {}

impl Context for MyFirstPluginRoot {}

impl RootContext for MyFirstPluginRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(MyFirstPlugin {}))
    }
}

struct MyFirstPlugin {}

impl Context for MyFirstPlugin {}

impl HttpContext for MyFirstPlugin {
    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        self.add_http_response_header("My-Plugin-Greeting", "Hello Proxy-Wasm");
        Action::Continue
    }
}
