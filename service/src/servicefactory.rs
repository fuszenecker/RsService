use infra::repositories::userrepository::UserRepository;

pub struct ServiceFactoryOptions<'a> {
    pub db_name: &'a str,
}

pub struct ServiceFactory<'a> {
    db_name: &'a str,
}

impl<'a> ServiceFactory<'a> {
    pub fn new(options: &ServiceFactoryOptions<'a>) -> Self {
        Self {
            db_name: options.db_name,
        }
    }

    pub fn create_bll(&mut self) -> bll::Bll {
        let persistence_layer = UserRepository::new(&self.db_name);
        bll::Bll::new(Box::new(persistence_layer))
    }
}
