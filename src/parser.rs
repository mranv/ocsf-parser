use crate::schema::OCSFEvent;
use serde_json::json;

pub fn parse_log_line(line: &str) -> Result<OCSFEvent, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = line.splitn(4, " ").collect();
    
    if parts.len() != 4 {
        return Err("Invalid log format".into());
    }

    let date_time = format!("{}T{}", parts[0], parts[1]);
    let severity = parts[2].to_uppercase();
    let message = parts[3].to_string();

    let (activity_id, activity_name, metadata) = parse_message(&message);

    Ok(OCSFEvent {
        class_uid: 1001, // Example: Generic log event
        category_uid: 2, // Example: System
        time: date_time,
        message,
        severity,
        activity_id,
        activity_name,
        metadata,
    })
}

fn parse_message(message: &str) -> (u32, String, Option<serde_json::Value>) {
    if message.starts_with("User login successful") {
        (1, "User Login".to_string(), Some(json!({"status": "success"})))
    } else if message.starts_with("Failed login attempt") {
        (1, "User Login".to_string(), Some(json!({"status": "failed"})))
    } else if message.starts_with("Database connection failed") {
        (2, "Database Connection".to_string(), Some(json!({"status": "failed"})))
    } else if message.starts_with("File upload completed") {
        (3, "File Upload".to_string(), Some(json!({"status": "completed"})))
    } else if message.starts_with("System shutdown initiated") {
        (4, "System Shutdown".to_string(), Some(json!({"reason": "high temperature"})))
    } else {
        (0, "Unknown".to_string(), None)
    }
}
