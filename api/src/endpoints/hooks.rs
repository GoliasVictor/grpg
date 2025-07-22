use super::prelude::*;
use actix_web::HttpRequest;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;
use std::process::Command;

// type HmacSha256 = Hmac<Sha256>;

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
    let signature_string = match req.headers().get("X-Hub-Signature-256") {
        Some(sig) => sig.to_str().unwrap_or(""),
        None => return HttpResponse::BadRequest().body("Bad Request: Missing signature"),
    };

    let signature = match hex::decode(signature_string.strip_prefix("sha256=").unwrap_or("")) {
        Ok(sig) => sig,
        Err(_) => return HttpResponse::BadRequest().body("Bad Request: Invalid signature format"),
    };

    let secret = match env::var("GITHUB_WEBHOOK_SECRET") {
        Ok(s) => s.as_bytes().to_vec(),
        Err(_) => return HttpResponse::InternalServerError().body("Secret not configured"),
    };
    
    if !verify_signature(&body, &signature, &secret) {
        return HttpResponse::Unauthorized().body("Invalid signature");
    }

    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to get current directory: {}", e)),
    };

    // Construct the path to the update script making it detached from current process with &
    let update_script = current_dir.parent().unwrap_or(&current_dir).parent().unwrap_or(&current_dir).join("update.sh");

    if !update_script.exists() {
        return HttpResponse::InternalServerError().body("Update script not found");
    }

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        
        let c = Command::new(&update_script)
            .current_dir(current_dir)
            .process_group(0)
            .spawn()
            .ok();
        if c.is_some() {
            return HttpResponse::Ok().body("Webhook processed successfully");
        } else {
            return HttpResponse::InternalServerError().body("Failed to spawn update script process");
        }
        
    }
    
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        let c = Command::new(&update_script)
            .creation_flags(DETACHED_PROCESS)
            .current_dir(current_dir)
            .spawn()
            .ok();
        
        if c.is_some() {
            return HttpResponse::Ok().body("Webhook processed successfully");
        } else {
            return HttpResponse::InternalServerError().body("Failed to spawn update script process");
        }
    }
}

fn verify_signature(payload: &[u8], signature: &[u8], secret: &[u8]) -> bool {
    // Create HMAC instance
    let mut mac = match Hmac::<Sha256>::new_from_slice(secret) {
        Ok(m) => m,
        Err(_) => return false,
    };
    
    // Update with payload
    mac.update(payload);

    // Verify signature
    return mac.verify_slice(&signature).is_ok();
}
