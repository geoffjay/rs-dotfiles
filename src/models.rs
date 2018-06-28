use super::schema::repos;

#[derive(Queryable)]
pub struct Repo {
    pub id: i32,
    pub name: String,
    pub url: String,
}

#[derive(Insertable)]
#[table_name = "repos"]
pub struct NewRepo<'a> {
    pub name: &'a str,
    pub url: &'a str,
}
