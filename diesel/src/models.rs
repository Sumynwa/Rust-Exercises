use serde::{Deserialize, Serialize};

use crate::schema::users;

//Diesel:Queryable - will generate code needed to load a User struct from a SQL Query.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
}
