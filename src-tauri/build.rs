fn main() {
    let app_id = std::env::var("APPLICATION_ID").expect("APPLICATION_ID is required");
    let auth_api_key = std::env::var("AUTH_API_KEY").expect("AUTH_API_KEY is required");

    println!("cargo:rustc-env=APPLICATION_ID={app_id}");
    println!("cargo:rustc-env=AUTH_API_KEY={auth_api_key}");

    tauri_build::build()
}
