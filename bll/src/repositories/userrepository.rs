use std::error::Error;

use crate::models::user::User;

pub trait UserRepository {
    fn save_user(&mut self, user: User) -> Result<(), Box<dyn Error>>;
    fn get_user(&mut self, user_id: i32) -> Result<User, Box<dyn Error>>;
}
