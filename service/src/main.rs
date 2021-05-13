mod servicefactory;

use actix_web::{get, web, App, HttpServer, Responder};
use servicefactory::{ServiceFactory, ServiceFactoryOptions };

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>, service_factory_options: web::Data<ServiceFactoryOptions>) -> impl Responder {
    let _bll = ServiceFactory::new(service_factory_options.get_ref())
        .create_bll();

    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
            App::new()
                .service(index)
                .data(ServiceFactoryOptions {})
        )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}