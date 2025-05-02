mod user;
use actix_web::{
    App, Error,
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    web::Data,
};
use nject::{injectable, provider};
use user::{ConnectionOptions, UserModule};

type Prov = Data<Provider>;

#[injectable]
#[provider]
pub struct Provider {
    #[import]
    user_mod: UserModule,
}

impl Provider {
    pub fn new(db_url: &str) -> Self {
        #[provider]
        struct InitProvider<'a> {
            #[provide]
            conn: ConnectionOptions<'a>,
        }

        let init = InitProvider {
            conn: ConnectionOptions { url: db_url },
        };
        init.provide()
    }
}

pub fn setup_app(
    provider: Prov,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = Error,
        InitError = (),
    > + 'static,
> {
    App::new().app_data(provider).service(user::create_scope())
}
