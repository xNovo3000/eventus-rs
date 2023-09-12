use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user_)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub password_: String,
    pub active: bool
}