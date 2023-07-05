

use super::database::*;
use super::database::DbError::*;
#[get("/documents/<id>")]
pub async fn ep_get_documents_id(id: &str) -> String {
    let result = match get_doc_by_id(id).await {
        Ok(doc) => doc,
        Err(er) => match er {
            EnvError(s) => return format!("Missing enironment variable:\n {s}").to_string(),
            EndPointError(er) => return format!("Failed to connect to database endpoint:\n {er}").to_string(),
            SignInError(er) => return format!("Failed to log in with provided db credentials:\n {er}").to_string(),
            SelectError(er) => return format!("Failed to get results of provided id:\n {er}").to_string(),
            _ => return "Error: Unexpected error returned from request".to_string()
        }
    };
    match serde_json::to_string(&result) {
        Ok(json) => format!("{{ \"status\": OK, \"result\": {json} }}").to_string(),
        Err(_) => "Failed to serialize database response".to_string()
    }
}

#[get("/documents")]
pub async fn ep_get_documents() -> String {
    let result = match get_docs().await {
        Ok(doc) => doc,
        Err(er) => match er {
            EnvError(s) => return format!("Missing enironment variable:\n {s}").to_string(),
            EndPointError(er) => return format!("Failed to connect to database endpoint:\n {er}").to_string(),
            SignInError(er) => return format!("Failed to log in with provided db credentials:\n {er}").to_string(),
            SelectError(er) => return format!("Failed to get results of provided id:\n {er}").to_string(),
            _ => return "Error: Unexpected error returned from request".to_string()
        }
    };
    match serde_json::to_string(&result) {
        Ok(json) => format!("{{ \"status\": OK, \"result\": {json} }}").to_string(),
        Err(_) => "Failed to serialize database response".to_string()
    }
}

use super::model::Document;
#[post("/documents", format = "application/json", data = "<doc_str>")]
pub async fn ep_post_documents(doc_str: &str) -> String {
    let doc: Document = match serde_json::from_str(doc_str) {
        Ok(x) => x,
        Err(_) => return "Error parsing POST data".to_string()
    };
    match post_doc(&doc, UpdateAction::Create).await {
        Ok(s) => s,
        Err(_e) => "Failed to post data to database lmao".to_string(),
    }
}

#[put("/documents/<id>", format = "application/json", data = "<doc_str>")]
pub async fn ep_put_documents(id: &str, doc_str: &str) -> String {
    let doc: Document = match serde_json::from_str(doc_str) {
        Ok(x) => x,
        Err(_) => return "Error parsing POST data".to_string()
    };
    match post_doc(&doc, UpdateAction::Modify(id.to_string())).await {
        Ok(s) => s,
        Err(_e) => "Failed to post data to database lmao".to_string(),
    }
}

#[delete("/documents/<id>")]
pub async fn ep_del_documents(id: &str) -> String {
    match delete_doc(id).await {
        Ok(success) => {
            if success {
                return "delete succesful".to_string();
            } else {
                return "document not found".to_string(); // Todo: turn this into a serialized datastructure
            }
        },
        Err(_e) => "Failed to post data to database lmao".to_string(),
    }
}