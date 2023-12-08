mod user;
use actix_web::{
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    web::Data,
    App, Error,
};
use nject::{injectable, provider};
use user::UserModule;

type Prov = Data<&'static Provider>;

#[injectable]
#[provider]
pub struct Provider {
    #[import]
    user_mod: UserModule,
}

impl Provider {
    pub fn new() -> Self {
        #[provider]
        struct InitProvider;

        InitProvider.provide()
    }
}

pub fn setup_app(
    provider: &'static Provider,
) -> App<
    impl ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse<impl MessageBody>,
            Error = Error,
            InitError = (),
        > + 'static,
> {
    App::new()
        .app_data(Data::new(provider))
        .service(user::create_scope())
}
