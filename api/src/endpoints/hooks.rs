use super::prelude::*;
use actix_web::HttpRequest;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;
use std::process::Command;
use std::thread;

type HmacSha256 = Hmac<Sha256>;

#[utoipa::path(
    request_body = (),
    responses(
        (status = 200, description = "Webhook processed successfully"),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[post("/hook")]
pub async fn github_webhook(req: HttpRequest, body: web::Bytes) -> impl Responder {
    let signature = match req.headers().get("X-Hub-Signature-256") {
        Some(sig) => sig.to_str().unwrap_or(""),
        None => return HttpResponse::BadRequest().body("Bad Request: Missing signature"),
    };

    let secret = match env::var("GITHUB_WEBHOOK_SECRET") {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().body("Secret not configured"),
    };
    
    if !verify_signature(&body, signature, &secret) {
        return HttpResponse::Unauthorized().body("Invalid signature");
    }

    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to get current directory: {}", e)),
    };

    let update_script = current_dir.parent().unwrap_or(&current_dir).parent().unwrap_or(&current_dir).join("update.sh");

    if !update_script.exists() {
        return HttpResponse::InternalServerError().body("Update script not found");
    }

    // Run the update script in a separate thread
    thread::spawn(move || {
        let output = Command::new(&update_script)
            .current_dir(current_dir)
            .output()
            .expect("Failed to execute update script");

        if !output.status.success() {
            eprintln!("Update script failed: {}", String::from_utf8_lossy(&output.stderr));
        }
    });
    
    return HttpResponse::Ok().body("Webhook processed successfully");
    
}

fn verify_signature(payload: &[u8], signature: &str, secret: &str) -> bool {
    let sig_str = signature.strip_prefix("sha256=").unwrap_or(signature);
    
    // Convert hex signature to bytes
    let sig_bytes = match hex::decode(sig_str) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    // Create HMAC instance
    let mut mac = match HmacSha256::new_from_slice(secret.as_bytes()) {
        Ok(m) => m,
        Err(_) => return false,
    };

    // Update with payload
    mac.update(payload);

    // Verify signature
    mac.verify_slice(&sig_bytes).is_ok()
}
