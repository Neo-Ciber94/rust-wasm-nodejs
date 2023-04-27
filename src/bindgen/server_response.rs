use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "http")]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(extends = js_sys::Object, js_name = ServerResponse, typescript_type = "ServerResponse")]
    pub type ServerResponse;

    #[wasm_bindgen(method, getter)]
    pub fn finished(this: &ServerResponse) -> bool;

    #[wasm_bindgen(method, getter, js_name = "statusCode")]
    pub fn status_code(this: &ServerResponse) -> u16;

    #[wasm_bindgen(method, setter, js_name = "statusCode")]
    pub fn set_status_code(this: &ServerResponse, value: u16);

    #[wasm_bindgen(method, getter, js_name = "statusMessage")]
    pub fn status_message(this: &ServerResponse) -> String;

    #[wasm_bindgen(method, setter, js_name = "statusMessage")]
    pub fn set_status_message(this: &ServerResponse, value: String);

    #[wasm_bindgen(method, js_name = "setHeader")]
    pub fn set_header(this: &ServerResponse, name: &str, value: &str);

    #[wasm_bindgen(method, catch, js_name = "writeHead")]
    pub fn write_head(
        this: &ServerResponse,
        status_code: u16,
        headers: JsValue,
    ) -> Result<ServerResponse, JsValue>;

    #[wasm_bindgen(method, catch, js_name = "writeHead")]
    pub fn write_head_with_status_message(
        this: &ServerResponse,
        status_code: u16,
        status_message: &str,
        headers: JsValue,
    ) -> Result<ServerResponse, JsValue>;

    #[wasm_bindgen(method, js_name = "write")]
    pub fn write(this: &ServerResponse, chunk: &[u8]) -> bool;

    #[wasm_bindgen(method, js_name = "write")]
    pub fn write_str(this: &ServerResponse, chunk: &str) -> bool;

    #[wasm_bindgen(method, js_name = "write")]
    pub fn write_string(this: &ServerResponse, chunk: String) -> bool;

    #[wasm_bindgen(method)]
    pub fn end(this: &ServerResponse);
}
