pub struct ServiceFactoryOptions<'a> {
    pub db_name: &'a str,
}

pub struct ServiceFactory<'a> {
    db_name: &'a str,
    user_repository: &'a mut Box<dyn bll::repositories::userrepository::UserRepository>
}

impl<'a> ServiceFactory<'a> {
    pub fn new(options: &ServiceFactoryOptions<'a>) -> Self {
        let persistence: &mut Box<dyn bll::repositories::userrepository::UserRepository> =
            &mut Box::new(infra::repositories::userrepository::UserRepository::new(options.db_name));

        Self {
            db_name: options.db_name,
            user_repository: persistence
        }
    }

    pub fn create_bll(&mut self) -> bll::Bll {
        bll::Bll::new(self.user_repository)
    }
}
