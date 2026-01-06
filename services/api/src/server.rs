use {
    actix_web::{
        body::MessageBody,
        dev::{Server, ServiceFactory, ServiceResponse},
        middleware, web, HttpServer, Result,
    },
    std::net::TcpListener,
};

use crate::status;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1").service(web::resource("/health").route(web::get().to(status::get_health))),
    );
}

pub fn create_app() -> actix_web::App<
    impl ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let app = actix_web::App::new().configure(configure);

    app.wrap(actix_web::middleware::Compress::default())
        .wrap(middleware::Logger::default())
}

pub fn run_http(listener: TcpListener) -> Result<Server, std::io::Error> {
    let http_server = HttpServer::new(move || create_app())
        .listen(listener)?
        .run();

    Ok(http_server)
}
