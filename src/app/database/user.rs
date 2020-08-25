use super::schema::users;

#[derive(Queryable, Identifiable)]
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
    use diesel::{RunQueryDsl, QueryDsl, prelude::*};
    use schema::users::{dsl, table};

    fn init() -> PgConnection {
        config::Config::load();
        connect()
    }

    fn get_seed() -> NewUser<'static> {
        NewUser {
            password: "myStrongPassword",
            username: "test@example21.com",
            name: None,
            surname: None,
        }
    }

    fn create_user() -> User {
        let user = get_seed();
        let connection = init();
        let created_user: User = diesel::insert_into(table)
            .values(&user)
            .get_result(&connection)
            .expect("Cannot create user!");
        assert_eq!(created_user.name.is_none(), user.name.is_none());
        assert_eq!(created_user.surname.is_none(), user.surname.is_none());
        assert_eq!(created_user.username.as_str(), user.username);
        assert_eq!(created_user.password.as_str(), user.password);
        assert!(created_user.id > 0);
        created_user
    }

    fn update_user(user: &User) {
        let connection = init();
        let updated_user: User = diesel::update(dsl::users.find(user.id))
            .set(dsl::name.eq(String::from("Tester")))
            .get_result(&connection)
            .expect("Cannot update user!");
        assert_eq!(updated_user.name.unwrap(), "Tester");
        assert_eq!(updated_user.surname.is_none(), user.surname.is_none());
        assert_eq!(updated_user.username.as_str(), user.username);
        assert_eq!(updated_user.password.as_str(), user.password);
        assert_eq!(updated_user.id, user.id);
    }

    fn get_users(id: i32) -> Vec<User> {
        let connection = init();
        dsl::users.find(id)
            .load::<User>(&connection)
            .expect("Cannot get user")
    }

    fn get_user(id: i32) {
        let selected_users = get_users(id);
        assert_eq!(selected_users.len(), 1);
        assert_eq!(selected_users[0].id, id);
    }

    fn delete_user(id: i32) {
        let connection = init();
        diesel::delete(dsl::users.find(id))
            .execute(&connection)
            .expect("Cannot get user");
        let selected_users = get_users(id);
        assert_eq!(selected_users.len(), 0);
    }

    #[test]
    fn user_flow() {
        let user = create_user();
        update_user(&user);
        get_user(user.id);
        delete_user(user.id);
    }
}
