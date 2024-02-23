use chrono::{DateTime, Local};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = crate::api::schema::eventus::user_)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_: String,
    pub active: bool
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::api::schema::eventus::authority)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Authority {
    pub id: i32,
    pub name_: String,
    pub user_id: i32
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::api::schema::eventus::event_)]
#[diesel(belongs_to(User, foreign_key = creator))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Event {
    pub id: i32,
    pub name_: String,
    pub description_: String,
    pub creator: i32,
    pub creation_date: DateTime<Local>,
    pub start_: DateTime<Local>,
    pub end_: DateTime<Local>,
    pub seats: i32,
    pub approved: bool
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::api::schema::eventus::subscription)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Event))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Subscription {
    pub id: i32,
    pub user_id: i32,
    pub event_id: i32,
    pub creation_date: DateTime<Local>,
    pub rating: Option<i32>,
    pub comment: Option<String>
}