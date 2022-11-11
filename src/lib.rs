use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn process() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    
    let textarea: web_sys::HtmlTextAreaElement = document.get_element_by_id("chat").unwrap().dyn_into().unwrap();
    let input = textarea.value();

    let colors = [
        "#EAEAEA",
        "#DDDDDD"
    ];

    let mut out = String::new();
    out.push_str("<div style=\"font: 16px courier, monospace; border-left: 5px solid #DDDDDD; max-width:750px\">\n");
    for (i, line) in input.trim().lines().enumerate() {
        let (first, rest) = line.split_once(":").unwrap_or((line,""));

        out += &format!("\t<p style=\"background: {}; margin: 1px; padding: 3px 4px;\"><b>{}:</b>{}</p>\n", colors[i % 2], first, rest);
    }
    out += "</div>";




    let preview = document.get_element_by_id("output").unwrap();
    preview.set_inner_html(&out);

    let output = document.get_element_by_id("code").unwrap();
    output.set_inner_html(&out);
    
    Ok(())
}