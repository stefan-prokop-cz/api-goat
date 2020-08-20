use super::schema::users;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub name: String,
    pub surname: String,
}

#[derive(Debug)]
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub name: Option<&'a str>,
    pub surname: Option<&'a str>,
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::super::super::{config};
    use crate::app::database::user::NewUser;
    use diesel::RunQueryDsl;

    #[test]
    fn create_user() {
        let user = NewUser {
            password: "myStrongPassword",
            username: "test@example.com",
            name: None,
            surname: None,
        };
        config::Config::load();
        let connection = connect();
        let _create_user = diesel::insert_into(schema::users::table)
            .values(user)
            .execute(&connection)
            .expect("Error");
    }
}
