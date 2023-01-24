use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

extern "C" {
    fn doubler(x: i32) -> i32;
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let doubled = unsafe { doubler(2) };
    println!("Hello, world: {doubled}");
}
