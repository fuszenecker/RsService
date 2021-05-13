pub mod repositories;
pub mod models;

use models::user::User;
use repositories::userrepository::UserRepository;

pub struct Bll {
    repository: Box<dyn UserRepository>
}

impl Bll {
    pub fn new(repository: Box<dyn UserRepository>) -> Self {
        Self {
            repository: repository,
        }
    }

    pub fn get_user_by_id(self: &mut Self, id: i32) -> Result<User, String> {
        self.repository.get_user(id)
    }

    pub fn create_user(self: &mut Self, id: i32, user_name: String) -> Result<(), String> {
        self.repository.save_user(User { id: id, name: user_name })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
