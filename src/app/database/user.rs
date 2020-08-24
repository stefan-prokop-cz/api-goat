use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub name: Option<String>,
    pub surname: Option<String>,
}

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
    use super::super::super::config;
    use super::super::*;
    use crate::app::database::user::{NewUser, User};
    use diesel::RunQueryDsl;

    fn init() -> PgConnection {
        config::Config::load();
        connect()
    }

    fn get_seed() -> NewUser<'static> {
        NewUser {
            password: "myStrongPassword",
            username: "test@example.com",
            name: None,
            surname: None,
        }
    }

    #[test]
    fn create_user() {
        use schema::users;
        let user = get_seed();
        let connection = init();
        let created_user: User = diesel::insert_into(users::table)
            .values(&user)
            .get_result(&connection)
            .expect("Cannot create user!");
        assert_eq!(created_user.name.is_none(), user.name.is_none());
        assert_eq!(created_user.surname.is_none(), user.surname.is_none());
        assert_eq!(created_user.username.as_str(), user.username);
        assert_eq!(created_user.password.as_str(), user.password);
        assert!(created_user.id > 0);
    }
}
