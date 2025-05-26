use wasm_bindgen::prelude::*;

#[derive(tsify::Tsify, serde::Deserialize)]
#[tsify(from_wasm_abi)]
pub struct Example {
    pub bar: String,
}

#[wasm_bindgen]
pub fn get_bar_via_tsify(example: Example) -> Result<String, JsValue> {
    Ok(example.bar)
}

#[wasm_bindgen]
pub fn get_bar_without_tsify(example: JsValue) -> Result<String, JsValue> {
    let example: Example = serde_wasm_bindgen::from_value(example)?;
    Ok(example.bar)
}
