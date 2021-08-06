use std::env;

use actix_web::{web::Data, App, HttpServer, web, middleware, Result};
use actix_web::http::ContentEncoding;
use actix_web_httpauth::middleware::HttpAuthentication;
use note_utils::MyError;
use crate::middleware::validator;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use crate::{users};

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

pub struct NoteContext {
    pub pool: DbPool,
}

impl NoteContext {
    pub fn create(
        pool: DbPool,
    ) -> NoteContext {
        NoteContext {
            pool,

        }
    }
    pub fn pool(&self) -> &DbPool {
        &self.pool
    }
}

impl Clone for NoteContext {
    fn clone(&self) -> Self {
        NoteContext {
            pool: self.pool.clone(),
        }
    }
}

pub async fn init() -> Result<(), MyError> {
    info!("{}", tr!("Starting server"));

    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");
    let db_url = env::var("DATABASE_URL").expect(tr!("Database url not set").as_str());

    let manager = ConnectionManager::<PgConnection>::new(&db_url);
    let pool = Pool::builder()
        .max_size(5)
        .build(manager)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));

    // let google_client_id =
    //     env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID is not set in .env file");
    //
    // let facebook_app_id =
    //     env::var("FACEBOOK_APP_ID").expect("FACEBOOK_APP_ID is not set in .env file");
    // let facebook_secret =
    //     env::var("FACEBOOK_SECRET").expect("FACEBOOK_SECRET is not set in .env file");
    // let facebook_access_token =
    //     env::var("FACEBOOK_ACCESS_TOKEN").expect("FACEBOOK_ACCESS_TOKEN is not set in .env file");
    //
    // let mut data = HashMap::new();
    //
    // data.insert("facebook_app_id".to_string(), facebook_app_id);
    // data.insert("facebook_secret".to_string(), facebook_secret);
    // data.insert("facebook_access_token".to_string(), facebook_access_token);


    HttpServer::new(move || {
        let context = NoteContext::create(
            pool.clone(),
        );
        App::new()
            .app_data(Data::new(context))
            .wrap(middleware::Compress::new(ContentEncoding::Gzip))
            .service(
                web::scope("/api")
                   // .wrap(HttpAuthentication::bearer(validator))
                    .configure(users::init_routes)
            )
    }).bind(format!("{}:{}", host, port))?
        .run().await?;
    Ok(())
}