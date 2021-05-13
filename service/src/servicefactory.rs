use infra::repositories::userrepository::{ UserRepository };

pub struct ServiceFactoryOptions {

}

pub struct ServiceFactory {

}

impl ServiceFactory {
    pub fn new(options: &ServiceFactoryOptions) -> Self {
        Self {  }
    }

    pub fn create_bll(self: &mut Self) -> bll::Bll {
        bll::Bll::new(Box::new(UserRepository::new()))
    }
}

