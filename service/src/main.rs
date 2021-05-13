mod servicefactory;

use actix_web::{get, web, App, HttpServer, Responder};
use servicefactory::{ServiceFactory, ServiceFactoryOptions };

#[get("/{id}")]
async fn index(web::Path(id): web::Path<i32>, service_factory_options: web::Data<ServiceFactoryOptions>) -> impl Responder {
    let mut bll = ServiceFactory::new(service_factory_options.get_ref())
        .create_bll();

    format!("Hello {}! id:{}", bll.get_user_by_id(id).unwrap().name, id)
}

#[get("/{id}/{name}")]
async fn create(web::Path((id, name)): web::Path<(i32, String)>, service_factory_options: web::Data<ServiceFactoryOptions>) -> impl Responder {
    let mut bll = ServiceFactory::new(service_factory_options.get_ref())
        .create_bll();

    bll.create_user(id, name.clone());

    format!("Hello {}! id:{}. Created.", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
            App::new()
                .service(index)
                .service(create)
                .data(ServiceFactoryOptions {
                    db_name: "data.sqlite".into()
                })
        )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}