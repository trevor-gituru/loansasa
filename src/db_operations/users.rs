use crate::models::users::{NewUser, User};
use diesel::prelude::*;

pub fn create_user(
    new_user: &NewUser, 
    conn: &mut PgConnection
) -> diesel::QueryResult<User>{
    use crate::schema::users::dsl::*;

     // normal diesel operations
     diesel::insert_into(users)
     .values(new_user)
     .get_result::<User>(conn)

}