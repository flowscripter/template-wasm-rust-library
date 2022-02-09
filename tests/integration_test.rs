use flowscripter_template_wasm_rust_library;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn world_works() {
    assert_eq!(4, flowscripter_template_wasm_rust_library::add(2, 2));
}
