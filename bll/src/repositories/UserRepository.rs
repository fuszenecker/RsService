use crate::models::user::User;

pub trait UserRepository {
    fn save_user(self: &mut Self, user: User) -> Result<(), String>;
    fn get_user(self: &mut Self, user_id: i32) -> Result<User, String>;
}