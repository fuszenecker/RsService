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

    pub fn process_user(self: &mut Self, user: User) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
