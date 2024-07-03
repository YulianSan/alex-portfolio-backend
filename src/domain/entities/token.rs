use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::tokens;

#[derive(Deserialize, Serialize, Clone, Queryable, Selectable)]
#[diesel(table_name = tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Token {
    pub id: i32,
    pub token: String,
    pub expire_at: NaiveDateTime,
    pub user_id: Uuid,
}
