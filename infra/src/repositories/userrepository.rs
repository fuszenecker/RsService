use bll::{models::user::User, repositories::userrepository::UserRepository as BllUserRepository};
use rusqlite::{params, Connection, OpenFlags, Result};

pub struct UserRepository {
    connection: Connection,
}

impl UserRepository {
    pub fn new(db_name: &str) -> Self {
        let connecion =
            rusqlite::Connection::open_with_flags(db_name, OpenFlags::SQLITE_OPEN_CREATE).unwrap();

        connecion
            .execute(
                "CREATE TABLE users (
                        id              INTEGER PRIMARY KEY,
                        name            TEXT NOT NULL
                )",
                [],
            )
            .unwrap();

        Self {
            connection: connecion,
        }
    }
}

impl BllUserRepository for UserRepository {
    fn save_user(&mut self, user: User) -> Result<(), String> {
        self.connection
            .execute(
                "INSERT INTO users (id, name) VALUES (?1, ?2)",
                params![user.id, user.name],
            )
            .unwrap();

        Ok(())
    }

    fn get_user(&mut self, _user_id: i32) -> Result<User, String> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, name FROM users")
            .unwrap();
        let users_iter = stmt
            .query_map([], |row| {
                Ok(User {
                    id: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                })
            })
            .unwrap();

        let user = users_iter.take(1).last().unwrap().unwrap();
        Ok(user)
    }
}
