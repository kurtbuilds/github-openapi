use std::path::Path;
use openapi_client_generator::{generate_library, generate_library_at_path, GenerateLibrary};

fn main() {
    generate_library_at_path(Path::new("openapi.yaml"), GenerateLibrary {
        name: "Github".to_string(),
        dest_path: "src".into(),
        lib_rs_path: Some("template/lib.rs".into()),
        model_rs_path: Some("template/model.rs".into()),
    }).unwrap();
}