use actix_web::{get, post, web::{Data,Json,Path}, App, HttpResponse, Responder};
use serde::{Deserialize,Serialize};
use sqlx::{self,FromRow};
use crate::AppState;

#[derive(Serialize,FromRow)]
struct Todo{
    id:i32,
    content:String,
    completed:bool
}
#[derive(Deserialize , Serialize , FromRow)]
struct NewTodo{
    content:String,
    completed:bool
}


#[get("/")]
pub async fn get_todos(state:Data<AppState>)->impl Responder{
    match sqlx::query_as::<_,Todo>("SELECT * FROM todos")
    .fetch_all(&state.db)
    .await{
        Ok(todos)=>HttpResponse::Ok().json(todos),
        Err(e)=>HttpResponse::NotFound().json("No users found")
    }    
}

#[post("/")]
pub async fn create_todo(state:Data<AppState>,body:Json<NewTodo>)->impl Responder{
    match sqlx::query_as::<_,NewTodo>("INSERT INTO todos (content,completed) VALUES ($1,$2) RETURNING *")
    .bind(body.content.clone())
    .bind(body.completed.clone())
    .fetch_one(&state.db)
    .await{
        Ok(todo)=>HttpResponse::Ok().json(todo),
        Err(e)=>HttpResponse::NotFound().json("No users found")
    }   

}
