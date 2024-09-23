use crate::models::user_details::{UserDetails, NewUserDetails};
use diesel::prelude::*;
use diesel::{QueryResult, insert_into};

pub fn create_user_details(
    new_user_details: &NewUserDetails, 
    conn: &mut PgConnection
) -> QueryResult<UserDetails>{
    use crate::schema::user_details::dsl::*;

     // normal diesel operations
     insert_into(user_details)
     .values(new_user_details)
     .get_result::<UserDetails>(conn)

}

pub fn update_user_details(
    new_user_details: &NewUserDetails,
    conn: &mut PgConnection
) -> QueryResult<UserDetails>{
    use crate::schema::user_details::dsl::*;

    diesel::update(user_details.filter(user_id.eq(new_user_details.user_id)))
    .set((
        account_address.eq(new_user_details.account_address),
        private_key.eq(new_user_details.private_key),
    ))
    .get_result::<UserDetails>(conn)

}
pub fn find_user_details(
    users_id: i32, 
    conn: &mut PgConnection
) -> QueryResult<UserDetails>{
    use crate::schema::user_details::dsl::*;

    return user_details.filter(user_id.eq(users_id)).first(conn);
}