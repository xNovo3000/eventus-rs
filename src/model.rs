use chrono::{DateTime, Local};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::user_)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub password_: String,
    pub active: bool
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::schema::authority)]
#[diesel(belongs_to(User))]
pub struct Authority {
    pub id: u32,
    pub name_: String,
    pub user_id: u32
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::schema::event_)]
#[diesel(belongs_to(User, foreign_key = creator))]
pub struct Event {
    pub id: u32,
    pub name_: String,
    pub description_: String,
    pub creator: u32,
    pub creation_date: DateTime<Local>,
    pub start_: DateTime<Local>,
    pub end_: DateTime<Local>,
    pub seats: i32,
    pub approved: bool
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::schema::subscription)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Event))]
pub struct Subscription {
    pub id: u32,
    pub user_id: u32,
    pub event_id: u32,
    pub creation_date: DateTime<Local>,
    pub rating: Option<i32>,
    pub comment: Option<String>
}