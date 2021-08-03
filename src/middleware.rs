use actix_web::{dev::{ServiceRequest}, Error};
use actix_web::http::{HeaderName, HeaderValue};

use crate::users::User;
use crate::utils::token_utils;
use crate::app::{NoteContext, DbPool};
use actix_web::web::Data;
use crate::db::{blocking};
use note_utils::MyError;
use anyhow::anyhow;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::bearer::{Config, BearerAuth};
use crate::constants;
use uuid::Uuid;

pub async fn validator(mut req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, actix_web::Error> {
        let mut authenticate_pass = false;
        let token = credentials.token();
        log::info!(" * Middleware Token: {}", token);
        if let Some(pool) = req.app_data::<Data<DbPool>>() {
                match token_utils::decode_token(token) {
                        Ok(token_data) => {
                                log::info!("Auth type: {}", token_data.claims.auth_type.clone());
                                let token_data_cl = token_data.claims.user.clone();
                                let user = blocking(pool, move |conn| {
                                        User::find_by_id(conn,&token_data_cl)
                                }).await?;

                                match user {
                                        Ok(user) => {
                                                let uid = user.user_id.unwrap();
                                                req.headers_mut().append(HeaderName::from_static(constants::AUTHENTICATED), HeaderValue::from_str(&uid).unwrap());

                                                authenticate_pass = true;
                                        }
                                        Err(_) => {
                                                log::warn!("* Middleware validator find not found user id => {:?}",&token_data.claims.user);
                                        }
                                }
                        }
                        Err(err) => {
                                log::warn!("* Middleware Decode token error: {}", err);
                        }
                }
        }


        if authenticate_pass {
                Ok(req)
        } else {
                let config = req.app_data::<Config>()
                    .map(|data| data.clone())
                    .unwrap_or_else(Default::default)
                    .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");

                Err(AuthenticationError::from(config).into())
        }
}
