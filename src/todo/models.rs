use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db;
use crate::error_handler::CustomError;
use crate::schema::todo;


#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "todo"]
pub struct Todo {
    pub content: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "todo"]
pub struct Todos {
    pub id: i32,
    pub content: String,
    pub done: bool,
}

impl Todo {
    fn from(todo: Todo) -> Todo {
        Todo {
            content: todo.content,
            done: todo.done,
        }
    }
}

impl Todos {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let todos = todo::table.load::<Todos>(&conn)?;
        Ok(todos)
    }
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let todo = todo::table.filter(todo::id.eq(id)).first(&conn)?;
        Ok(todo)
    }

    pub fn create(todo: Todo) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let todo = Todo::from(todo);
        let todo = diesel::insert_into(todo::table).values(todo).get_result(&conn)?;
        Ok(todo)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(todo::table.filter(todo::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}