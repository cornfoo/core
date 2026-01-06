use actix_web::{web, HttpResponse, Result};
use primitives::Device;
use storage::{DeviceClient, PoolBuilder, UpdateDeviceRow};

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
            tracing::error!("Failed to add device: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "failed to add device",
                "message": e.to_string()
            })))
        }
    }
}
