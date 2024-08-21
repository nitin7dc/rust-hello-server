use actix_web::{get, post, patch, delete, web, HttpResponse, Responder};
use mongodb::{Database, bson::doc, Collection};
use crate::models::{User, CreateUserRequest, UpdateUserRequest};
use futures::stream::TryStreamExt;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/users")]
async fn create_user(db: web::Data<Database>, new_user: web::Json<CreateUserRequest>) -> impl Responder {
        
    let collection: Collection<User> = db.collection("users");

    let user = User {
        id: None,
        name: new_user.name.clone(),
        mobile: new_user.mobile.clone(),
        email: new_user.email.clone(),
    };

    match collection.insert_one(user).await {
        Ok(result) => {
            let user_id = result.inserted_id.as_object_id().unwrap();
            match collection.find_one(doc! { "_id": user_id }).await {
                Ok(Some(created_user)) => HttpResponse::Created().json(created_user),
                _ => HttpResponse::InternalServerError().body("Failed to fetch created user"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
    }

}

#[get("/users/{id}")]
async fn get_user(db: web::Data<Database>, id: web::Path<String>) -> impl Responder {
    
    let collection: Collection<User> = db.collection("users");

    match mongodb::bson::oid::ObjectId::parse_str(id.as_str()) {
        Ok(object_id) => {
            match collection.find_one(doc! { "_id": object_id }).await {
                Ok(Some(user)) => HttpResponse::Ok().json(user),
                Ok(None) => HttpResponse::NotFound().body("User not found"),
                Err(_) => HttpResponse::InternalServerError().body("Failed to fetch user"),
            }
        }
        Err(_) => HttpResponse::BadRequest().body("Invalid User ID"),
    }

}

#[get("/users")]
async fn get_all_users(db: web::Data<Database>) -> impl Responder {

    let collection: Collection<User> = db.collection("users");

    match collection.find(doc!{}).await {
        Ok(cursor) => {
            match cursor.try_collect::<Vec<User>>().await {
                Ok(users) => HttpResponse::Ok().json(users),
                Err(_) => HttpResponse::InternalServerError().body("Failed to fetch users"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch users"),
    }
    
}

#[patch("/users/{id}")]
async fn update_user(db: web::Data<Database>, id: web::Path<String>, user_update: web::Json<UpdateUserRequest>) -> impl Responder {
    
    let collection: Collection<User> = db.collection("users");

    match mongodb::bson::oid::ObjectId::parse_str(id.as_str()) {
        Ok(object_id) => {
            let update = doc! {
                "$set": {
                    "name": &user_update.name,
                    "email": &user_update.email,
                }
            };
            match collection.find_one_and_update(doc! { "_id": object_id }, update).await {
                Ok(Some(updated_user)) => HttpResponse::Ok().json(updated_user),
                Ok(None) => HttpResponse::NotFound().body("User not found"),
                Err(_) => HttpResponse::InternalServerError().body("Failed to update user"),
            }
        }
        Err(_) => HttpResponse::BadRequest().body("Invalid User ID"),
    }
}

#[delete("/users/{id}")]
async fn delete_user(db: web::Data<Database>, id: web::Path<String>) -> impl Responder {
   
    let collection: Collection<User> = db.collection("users");

    match mongodb::bson::oid::ObjectId::parse_str(id.as_str()) {
        Ok(object_id) => {
            match collection.delete_one(doc! { "_id": object_id }).await {
                Ok(result) if result.deleted_count == 1 => HttpResponse::NoContent().finish(),
                Ok(_) => HttpResponse::NotFound().body("User not found"),
                Err(_) => HttpResponse::InternalServerError().body("Failed to delete user"),
            }
        }
        Err(_) => HttpResponse::BadRequest().body("Invalid User ID"),
    }
}