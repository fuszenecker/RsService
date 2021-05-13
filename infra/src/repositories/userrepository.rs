use bll::{models::user::User, repositories::userrepository::UserRepository as BllUserRepository};

pub struct UserRepository {

}

impl UserRepository {
    pub fn new() -> Self { Self {  } }
}

impl BllUserRepository for UserRepository {
    fn save_user(self: &mut Self, _user: User) -> Result<(), String> {
        todo!()
    }

    fn get_user(self: &mut Self, _user_id: i32) -> Result<User, String> {
        todo!()
    }
}

