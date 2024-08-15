use crate::models::users::{NewUser, User};
use diesel::prelude::*;
use diesel::{QueryResult, insert_into};
use regex::Regex;

pub fn create_user(
    new_user: &NewUser, 
    conn: &mut PgConnection
) -> QueryResult<User>{
    use crate::schema::users::dsl::*;

     // normal diesel operations
     insert_into(users)
     .values(new_user)
     .get_result::<User>(conn)

}

pub fn find_user(
    field: &str, 
    conn: &mut PgConnection
) -> QueryResult<User>{
    use crate::schema::users::dsl::*;

    // Check if the string is an unsigned integer
    let email_regex = Regex::new(r"^[\w\.-]+@[a-zA-Z\d\.-]+\.[a-zA-Z]{2,}$").unwrap();
    // Check if field is integer
    if field.parse::<i32>().is_ok() {
        let user_id = field.parse::<i32>().unwrap();
        return users.filter(id.eq(user_id)).first(conn);
    }

    else if email_regex.is_match(field) {
        return users.filter(email.eq(field)).first(conn);
    }
    // Assume its a username
    else
    {
        return users.filter(name.eq(field)).first(conn);
    }
}
pub fn verify_password(test_user: &User, test_passw: &str)-> bool{
    if test_user.password == test_passw{
        true
    }
    else
    {
        false
    }
}