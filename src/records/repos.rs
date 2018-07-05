use diesel;
use diesel::prelude::*;

use database;
use schema::repos;
use models::{Repo, NewRepo};

impl Repo {
    pub fn create(name: &str, url: &str) -> Self {
        let conn = database::establish_connection();
        let new_repo = NewRepo {
            name: name,
            url: url,
        };

        diesel::insert_into(repos::table)
            .values(&new_repo)
            .execute(&conn)
            .expect("Error saving new repo");

        Self::last()
    }

    pub fn all() -> Box<Vec<Self>> {
        use schema::repos::dsl::*;

        let conn = database::establish_connection();
        let results = repos
            .load::<Self>(&conn)
            .expect("Error loading repos");

        // XXX: Not sure if this is the correct structure?
        Box::new(results)
    }

    pub fn delete(record: i32) -> usize {
        use schema::repos::dsl::*;

        let conn = database::establish_connection();

        // TODO: This should delete associated profiles as well

        diesel::delete(repos.filter(id.eq(record)))
            .execute(&conn)
            .expect("error deleting repo")
    }

    pub fn last() -> Self {
        use self::repos::dsl::{repos, id};

        let conn = database::establish_connection();

        repos.order(id.desc()).first(&conn).unwrap()
    }
}
