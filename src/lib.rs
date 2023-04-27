mod bindgen;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bindgen::{ClientRequest, ServerResponse};
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast, JsError,
};

#[wasm_bindgen]
pub fn handle_request(req: ClientRequest, res: ServerResponse) -> Result<(), JsError> {
    let headers = serde_wasm_bindgen::from_value::<HashMap<String, String>>(req.headers())
        .map_err(|err| JsError::new(&err.to_string()))?;

    if let Some(content_type) = headers.get("content-type") {
        if !content_type.starts_with("text/plain") {
            res.set_status_code(400);
            res.write_string(format!("Expected text/plain but was {content_type}"));
            res.end();
            return Ok(());
        }
    }

    // Buffer to store the response stream
    let buffer = Arc::new(Mutex::new(vec![]));

    // To handle the incoming data
    let on_data = Closure::wrap({
        let body = buffer.clone();
        Box::new(move |buf| {
            // Write the received data
            body.lock().unwrap().extend(buf);
        }) as Box<dyn FnMut(Vec<u8>)>
    });

    // Register `data` handler
    req.on(
        bindgen::RequestEvent::Data,
        on_data.as_ref().unchecked_ref(),
    );

    // To handle the end of the data
    let on_end = Closure::wrap(Box::new(move || {
        // We take the buffer and reverse the string
        let buffer = buffer.lock().unwrap();
        let body = buffer.iter().rev().cloned().collect::<Vec<_>>();

        res.set_status_code(200);
        res.set_status_message("OK".to_owned());
        res.set_header("Content-Type", "text/plain, charset=utf-8");

        if body.is_empty() {
            res.write_str("There is no message, try POST instead");
        } else {
            res.write(&body);
        }

        res.end();
    }) as Box<dyn FnMut()>);

    // Register `end` handler
    req.on(bindgen::RequestEvent::End, on_end.as_ref().unchecked_ref());

    // We should forget, closure can live even more than the function call:
    //  https://rustwasm.github.io/docs/wasm-bindgen/examples/closures.html#srclibrs
    on_data.forget();
    on_end.forget();

    Ok(())
}
