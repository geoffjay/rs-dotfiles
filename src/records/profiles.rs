use diesel;
use diesel::prelude::*;

use database;
use schema::profiles;
use models::{Profile, NewProfile};

impl Profile {
    pub fn create(name: &str, repo_id: i32) -> Self {
        let conn = database::establish_connection();
        let new_profile = NewProfile {
            name: name,
            repo_id: repo_id,
        };

        diesel::insert_into(profiles::table)
            .values(&new_profile)
            .execute(&conn)
            .expect("Error saving new profile");

        Self::last()
    }

    pub fn all() -> Box<Vec<Self>> {
        use schema::profiles::dsl::*;

        let conn = database::establish_connection();
        let results = profiles
            .load::<Self>(&conn)
            .expect("Error loading profiles");

        // XXX: Not sure if this is the correct structure?
        Box::new(results)
    }

    pub fn delete(record: i32) -> usize {
        use schema::profiles::dsl::*;

        let conn = database::establish_connection();

        diesel::delete(profiles.filter(id.eq(record)))
            .execute(&conn)
            .expect("error deleting profile")
    }

    pub fn last() -> Self {
        use self::profiles::dsl::{profiles, id};

        let conn = database::establish_connection();

        profiles.order(id.desc()).first(&conn).unwrap()
    }
}
