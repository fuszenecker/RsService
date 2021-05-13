pub mod models;
pub mod repositories;

use std::error::Error;

use models::user::User;
use repositories::userrepository::UserRepository;

pub struct Bll<'a> {
    repository: &'a mut Box<dyn UserRepository>,
}

impl<'a> Bll<'a> {
    pub fn new(repository: &'a mut Box<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub fn get_user_by_id(&mut self, id: i32) -> Result<User, Box<dyn Error>> {
        self.repository.get_user(id)
    }

    pub fn create_user(&mut self, id: i32, user_name: String) -> Result<(), Box<dyn Error>> {
        self.repository.save_user(User {
            id,
            name: user_name,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
