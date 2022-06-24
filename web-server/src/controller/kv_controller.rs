use rocket_contrib::json;
use crate::configuration::response::{ApiError, ApiResponse, success};
use storage::parquet_storage::ParquetStorage;
use storage::reader::Reader;
use storage::writer::Writer;

#[get("/demo")]
pub fn save() -> Result<ApiResponse, ApiError> {
    let storage_obj = ParquetStorage {};
    storage_obj.write("a".to_string(), "b".to_string());
    Ok(success(json!({
        "id": "mocked_id".to_string(),
        "name": "mocked_name".to_string(),
    })))
}

#[get("/read")]
pub fn read() -> Result<ApiResponse, ApiError> {
    let storage_obj = ParquetStorage {};
    storage_obj.read_by_key("a".to_string());
    Ok(success(json!({
        "id": "mocked_id".to_string(),
        "name": "mocked_name".to_string(),
    })))
}