use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Write};
use wasm_bindgen::prelude::*;
use zip::write::{ExtendedFileOptions, FileOptions};
use zip::ZipWriter;

#[wasm_bindgen]
pub async fn compress_files_to_zip(files: JsValue) -> Result<Uint8Array, JsError> {
    // 1. Convert JsValue (likely an array of objects) to a usable Rust structure
    let file_data_list: Vec<FileData> = serde_wasm_bindgen::from_value(files)
        .map_err(|e| JsError::new(&format!("Failed to deserialize file data: {}", e)))?;

    // 2. Create an in-memory buffer to write the ZIP archive to
    let buffer = Cursor::new(Vec::new());
    let mut zip_writer = ZipWriter::new(buffer);

    let options = FileOptions::<ExtendedFileOptions>::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755); // Optional: Set permissions for files in the ZIP

    // 3. Iterate through each file data and add it to the ZIP archive
    for file_data in file_data_list {
        let file_name = file_data.name;
        let file_content = file_data.content;

        zip_writer.start_file(file_name, options.clone())?;

        // Convert Uint8Array (from JsValue) to &[u8] for writing to ZipWriter
        zip_writer.write_all(&file_content)?;
    }

    // 4. Finish writing the ZIP archive and get the buffer
    let zip_buffer = zip_writer.finish()?.into_inner();

    // 5. Convert the Vec<u8> buffer to Uint8Array for returning to JavaScript
    let uint8_array = Uint8Array::from(zip_buffer.as_slice());
    Ok(uint8_array)
}

// Define a struct to represent file data received from JavaScript
#[derive(Deserialize, Serialize)]
struct FileData {
    name: String,
    #[serde(with = "serde_bytes")]
    content: Vec<u8>,
}
