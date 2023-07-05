use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use std::env;

#[derive(Debug)]
pub enum DbError {
    EnvError(String),
    EndPointError(surrealdb::Error),
    SignInError(surrealdb::Error),
    SelectError(surrealdb::Error),
    CreateError(surrealdb::Error),
    SerDeError,
}

pub enum UpdateAction {
    Create,
    Modify(String),
}

pub async fn connect() -> Result<Surreal<Client>, DbError> {
    let database_endpoint = env::var("DATABASE_ENDPOINT");
    let db_string: String;
    match database_endpoint {
        Ok(x) => db_string = x,
        Err(_) => return Err(DbError::EnvError("DATABASE_ENDPOINT".to_string())),
    }

    let connected = match Surreal::new::<Ws>(db_string).await {
        Ok(db) => db,
        Err(er) => return Err(DbError::EndPointError(er)),
    };
     

    let user =  match env::var("DB_USER") {
        Ok(u) => u,
        Err(_) => return Err(DbError::EnvError("DB_USER".to_string())),
    };

    let pass = match env::var("DB_PASS") {
        Ok(p) => p,
        Err(_) => return Err(DbError::EnvError("DB_PASS".to_string())),
    };

    match connected.signin( Root {
        username: &user, password: &pass,
    }).await {
        Ok(_) => Ok(connected),
        Err(er) => Err(DbError::SignInError(er)),
    }

}


use super::model::*;
pub async fn get_doc_by_id(id: &str) -> Result<RespDocument, DbError> {
    let db = match connect().await {
        Ok(c) => c,
        Err(er) => return Err(er),
    };
    db.use_ns("test").use_db("test").await.unwrap();
    let res:Result<RespDocument, surrealdb::Error> =  db.select(("documents", id)).await;
    match res {
        Ok(doc) => Ok(doc),
        Err(err) => Err(DbError::SelectError(err))
    }
}

pub async fn get_docs() -> Result<Vec<RespDocument>, DbError> {
    let db = match connect().await {
        Ok(c) => c,
        Err(er) => return Err(er),
    };
    db.use_ns("test").use_db("test").await.unwrap();
    let res:Result<Vec<RespDocument>, surrealdb::Error> = db.select("documents").await;
    match res {
        Ok(doc) => Ok(doc),
        Err(err) => Err(DbError::SelectError(err))
    }
}


pub async fn post_doc(doc: &Document, action: UpdateAction) -> Result<String, DbError> {
    let db = match connect().await {
        Ok(c) => c,
        Err(er) => return Err(er),
    };

    // I'm just gonna reuse this function for creating a new document and updating an existing one.
    db.use_ns("test").use_db("test").await.unwrap();
    let response = match action {
            UpdateAction::Create => db.create("documents").content(doc).await,
            UpdateAction::Modify(id) => db.update(("documents", id)).content(doc).await
    };
    
    let new_doc: RespDocument = match response {
        Ok(v) => v,
        Err(err) => return Err(DbError::CreateError(err)) // Todo: This probably isn't a good name for this error then.
    };
    return match serde_json::to_string(&new_doc) {
        Ok(s) => Ok(s),
        Err(_e) => Err(DbError::SerDeError)
    };
}

pub async fn delete_doc(id: &str) -> Result<bool, DbError> {
        let db = match connect().await {
        Ok(c) => c,
        Err(er) => return Err(er),
    };
    db.use_ns("test").use_db("test").await.unwrap();
    let res:Result<Option<RespDocument>, surrealdb::Error> =  db.select(("documents", id)).await;
    match res {
        Ok(some_doc) => match some_doc {
            Some(_doc) => return Ok(true),
            None => return Ok(false),
        },
        Err(err) => Err(DbError::SelectError(err))
    }
}
