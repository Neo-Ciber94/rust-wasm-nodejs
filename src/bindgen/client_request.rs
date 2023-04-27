use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum RequestEvent {
    Data = "data",
    End = "end",
}

#[wasm_bindgen(module = "http")]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(extends = js_sys::Object, js_name = ClientRequest, typescript_type = "ClientRequest")]
    pub type ClientRequest;

    #[wasm_bindgen(method, getter)]
    pub fn url(this: &ClientRequest) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn headers(this: &ClientRequest) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn method(this: &ClientRequest) -> String;

    #[wasm_bindgen(method, getter, js_name = "statusCode")]
    pub fn status_code(this: &ClientRequest) -> Option<u16>;

    #[wasm_bindgen(method, getter, js_name = "statusMessage")]
    pub fn status_message(this: &ClientRequest) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = "httpVersion")]
    pub fn http_version(this: &ClientRequest) -> String;

    #[wasm_bindgen(method, js_name = "on")]
    pub fn on(this: &ClientRequest, event: RequestEvent, callback: &js_sys::Function);
}
