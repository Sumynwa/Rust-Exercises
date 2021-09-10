use diesel::prelude::*;
use uuid::Uuid;

use crate::models;

pub fn find_user_by_id(uid: Uuid, conn: &SqliteConnection) -> Result<Option<models::User>, diesel::result::Error> {
    // this line imports a bunch of aliases so that we can say user instead of user::table.
    use crate::schema::users::dsl::*;

    let user = users.filter(id.eq(uid.to_string())).first::<models::User>(conn).optional()?;
    
    Ok(user)
}

pub fn insert_new_user(nm: &str, conn: &SqliteConnection) -> Result<models::User, diesel::result::Error> {
    // import schema related module inside a functions scope (rather than normal module's scope) to prevent import collisions/namespace pollution
    use crate::schema::users::dsl::*;

    let new_user = models::User{id: Uuid::new_v4().to_string(),
                                name: nm.to_owned(),};

    //does this call block?
    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}
