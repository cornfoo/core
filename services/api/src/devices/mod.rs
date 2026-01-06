use actix_web::{web, HttpResponse, Result};
use primitives::Device;
use storage::{DbPoolError, DeviceClient, PoolBuilder, UpdateDeviceRow};

pub async fn get_device(
    pool: web::Data<PoolBuilder>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let device_id = path.into_inner();
    let client = DeviceClient::new(&pool);

    match client.get_device(&device_id).await {
        Ok(row) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "data": row.as_primitive(),
        }))),
        Err(DbPoolError::NotFound) => {
            Ok(HttpResponse::NotFound().json(serde_json::json!({
                "error": "device not found",
            })))
        }
        Err(e) => {
            tracing::error!(error = ?e, "Failed to get device");
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "failed to get device",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn add_devices(
    pool: web::Data<PoolBuilder>,
    device: web::Json<Device>,
) -> Result<HttpResponse> {
    let client = DeviceClient::new(&pool);
    let device_row = UpdateDeviceRow::from_primitive(device.into_inner());

    match client.add_device(device_row).await {
        Ok(row) => Ok(HttpResponse::Created().json(serde_json::json!({
            "data": row.as_primitive(),
            "message": "device created successfully",
            "code": 201
        }))),
        Err(e) => {
            tracing::error!(error = ?e, "Failed to add device");
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "failed to add device",
                "message": e.to_string()
            })))
        }
    }
}
