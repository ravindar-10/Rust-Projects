use std::sync::MutexGuard;

use super::models::{CreateEntryData, UpdateEntryData};
use crate::{AppState, TodolistEntry};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/todolist/entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.todolist_entries.lock().unwrap().to_vec())
}
#[get("/todolist/entries/{id}")]
async fn get_entries_by_id(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let todolist_entries = data.todolist_entries.lock().unwrap();
    let id = path.into_inner();
    let filtered_entries: Vec<TodolistEntry> = todolist_entries
        .to_vec()
        .into_iter()
        .filter(|x| x.id == id)
        .collect();
    HttpResponse::Ok().json(filtered_entries.to_vec())
}

#[post("/todolist/entries")]
async fn create_entry(
    data: web::Data<AppState>,
    param_obj: web::Json<CreateEntryData>,
) -> impl Responder {
    let mut todolist_entries: MutexGuard<'_, Vec<TodolistEntry>> =
        data.todolist_entries.lock().unwrap();
    let mut max_id: i32 = 0;
    //get the max_id from all the entries
    for entry in 0..todolist_entries.len() {
        if todolist_entries[entry].id > max_id {
            max_id = todolist_entries[entry].id;
        }
    }
    //Insert the entry in after increasing the max_id by 1
    todolist_entries.push(TodolistEntry {
        id: max_id + 1,
        title: param_obj.title.clone(),
        date: param_obj.date,
    });
    //return the json response
    HttpResponse::Ok().json(todolist_entries.to_vec())
}
#[put("/todolist/entries/{id}")]
async fn update_entry(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    param_obj: web::Json<UpdateEntryData>,
) -> impl Responder {
    let id = path.into_inner();
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    let mut id_present = false;
    for i in 0..todolist_entries.len() {
        if todolist_entries[i].id == id {
            id_present = true;
            todolist_entries[i].title = param_obj.title.clone();
            break;
        }
    }
    if id_present {
        HttpResponse::Ok().json(todolist_entries.to_vec())
    } else {
        HttpResponse::BadRequest().body(format!(
            "Warning: ID: {} is not present in the todo list entries",
            id
        ))
    }
}

#[delete("/todolist/entries/{id}")]
async fn delete_entry(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    let id = path.into_inner();
    *todolist_entries = todolist_entries
        .to_vec()
        .into_iter()
        .filter(|x| x.id != id)
        .collect();
    HttpResponse::Ok().json(todolist_entries.to_vec())
}
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_entries)
        .service(get_entries_by_id)
        .service(create_entry)
        .service(update_entry)
        .service(delete_entry);
}
