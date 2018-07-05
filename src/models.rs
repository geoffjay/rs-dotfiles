//extern crate diesel;

use schema::{repos, profiles};
//use diesel::deserialize::Queryable;

//type DB = diesel::sqlite::Sqlite;

#[derive(Queryable)]
pub struct Repo {
    pub id: i32,
    pub name: String,
    pub url: String,
}

#[derive(Identifiable, Associations, Queryable)]
#[belongs_to(Repo)]
pub struct Profile {
    pub id: i32,
    pub repo_id: i32,
    pub name: String,
}

//impl Queryable<repos::SqlType, DB> for Repo {
    //type Row = (i32, String, String);

    //fn build(row: Self::Row) -> Self {
        //Repo {
            //id: row.0,
            //name: row.1.to_lowercase(),
            //url: row.2.to_lowercase(),
        //}
    //}
//}

#[derive(Insertable)]
#[table_name = "repos"]
pub struct NewRepo<'a> {
    pub name: &'a str,
    pub url: &'a str,
}

#[derive(Insertable)]
#[table_name = "profiles"]
pub struct NewProfile<'a> {
    pub repo_id: i32,
    pub name: &'a str,
}
