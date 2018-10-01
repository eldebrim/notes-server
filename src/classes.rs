#![allow(proc_macro_derive_resolution_fallback)]
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

use std::fmt;

use schema::classes;
use schema::classes::dsl::classes as all_classes;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "classes"]
pub struct Class {
    pub id: i32,
    pub title: String,
    pub notes: String,
}
#[derive(Insertable)]
#[table_name = "classes"]
pub struct NewClass<'a> {
    pub title: String,
    pub notes: &'a str,
}
impl<'a> Class {
    pub fn create_class(conn: &PgConnection, title: String, notes: &'a str) -> Class {
        use schema::classes;
        let new_class = NewClass {
            title: title,
            notes: notes,
        };

        diesel::insert_into(classes::table)
            .values(&new_class)
            .get_result(conn)
            .expect("Error saving new Class")
    }
    pub fn list(conn: &PgConnection) -> Vec<Class> {
        all_classes
            .order(classes::id.desc())
            .load::<Class>(conn)
            .expect("Error loading classes list")
    }
    pub fn show(id: i32, conn: &PgConnection) -> Vec<Class> {
        all_classes
            .find(id)
            .load::<Class>(conn)
            .expect("Error loading Class")
    }
    pub fn delete(id: i32, conn: &PgConnection) -> bool {
        if Class::show(id, conn).is_empty() {
            return false;
        };
        diesel::delete(all_classes.find(id)).execute(conn).is_ok()
    }
    pub fn display_string(classes: Vec<Class>) -> String {
        let mut table = String::new();
        for class in classes {
            table.push_str(
                &[
                    "\nID: ".to_string(),
                    class.id.to_string(),
                    "Title: ".to_string(),
                    class.title.clone(),
                    "Note: ".to_string(),
                    class.notes.clone(),
                ]
                    .join("\t"),
            );
            table.push_str("\n");
        }
        table
    }
}
