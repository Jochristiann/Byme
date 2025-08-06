use std::fs::File;
use std::io::Write;
use base64::Engine;
use base64::engine::general_purpose;
use uuid::Uuid;

pub fn save_base64_image_to_folder(base64_str: &str, folder_path: &str, id:Uuid) -> String {
    // Strip metadata header if present (e.g. "data:image/png;base64,....")
    let base64_data = if let Some(comma_index) = base64_str.find(',') {
        &base64_str[comma_index + 1..]
    } else {
        base64_str
    };

    // Decode base64 using the new engine API
    let image_data = general_purpose::STANDARD
        .decode(base64_data)
        .expect("Failed to decode base64 image");

    // Ensure the folder exists
    std::fs::create_dir_all(folder_path).expect("Failed to create folder");

    // Generate a unique filename
    let filename = format!("{}.png", id);
    let filepath = format!("{}/{}", folder_path, filename);

    // Write the image to a file
    let mut file = File::create(&filepath).expect("Failed to create file");
    file.write_all(&image_data).unwrap();

    let frontend_path = format!("{}/{}", folder_path, filename);
    frontend_path
}