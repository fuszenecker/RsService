use infra::repositories::userrepository::{ UserRepository };

pub struct ServiceFactoryOptions {
    pub db_name: String
}

pub struct ServiceFactory {
    db_name: String
}

impl ServiceFactory {
    pub fn new(options: &ServiceFactoryOptions) -> Self {
        Self {
            db_name: options.db_name.clone()
         }
    }

    pub fn create_bll(self: &mut Self) -> bll::Bll {
        bll::Bll::new(Box::new(UserRepository::new(self.db_name.clone())))
    }
}

