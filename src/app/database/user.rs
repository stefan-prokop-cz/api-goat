use super::super::database::connect;
use super::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub name: Option<String>,
    pub surname: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub name: Option<String>,
    pub surname: Option<String>,
}

pub fn create_user(user: &NewUser) -> User {
    let connection = connect();
    let created_user: User = diesel::insert_into(users::table)
        .values(user)
        .get_result(&connection)
        .expect("Cannot create user!");
    created_user
}

#[cfg(test)]
mod tests {
    use super::super::super::config;
    use super::super::*;
    use super::create_user as create_user_fn;
    use crate::app::database::user::{NewUser, User};
    use diesel::{prelude::*, QueryDsl, RunQueryDsl};
    use schema::users::{dsl, table};

    fn init() -> PgConnection {
        config::Config::load();
        connect()
    }

    fn get_seed() -> NewUser {
        NewUser {
            password: String::from("myStrongPassword"),
            username: String::from("test@example21.com"),
            name: None,
            surname: None,
        }
    }

    fn create_user() -> User {
        let user = get_seed();
        let connection = init();
        let created_user = create_user_fn(&user);
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
        dsl::users
            .find(id)
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
