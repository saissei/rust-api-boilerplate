use super::models::sample_body::ResponseSample;
use actix_web::{get, HttpResponse, Responder};
#[derive(Debug, Serialize, Deserialize)]
struct ResponseData {
  message: String,
}

#[get("/")]
pub async fn root() -> impl Responder {
  HttpResponse::Ok().body("OK")
}

#[get("/again")]
pub async fn again() -> HttpResponse {
  let res_data = r#"{
    "message": "Hello world"
  }"#;
  let serialized: ResponseSample = serde_json::from_str(res_data).unwrap();

  return HttpResponse::Ok().json(serialized);
}
