#[macro_use]
extern crate serde;

mod error;
mod models;
mod account;
mod service;

pub use error::*;
pub use models::*;
pub use account::*;
pub use service::*;

#[cfg(test)]
mod tests {
    use super::*;

    const API_KEY: &str = "<APT_KEY>";

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_service() {
        let service = Service::new(API_KEY);
        println!("{:?}", service);
    }

    #[tokio::test]
    async fn test_refresh_token() {
        let service = Service::new(API_KEY);
        let refresh_token = "<REFRESH_TOKEN>";
        match service.refresh_token(refresh_token).await {
            Ok(response) => {
                println!("{:?}", response);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

    #[tokio::test]
    async fn test_lookup() {
        let service = Service::new(API_KEY);
        let id_token = "<ID_TOKEN>";
        match service.lookup(id_token).await {
            Ok(response) => {
                println!("{:?}", response);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

    #[tokio::test]
    async fn test_sign_in_with_idp() {
        let service = Service::new(API_KEY);
        let access_token = "<ACCESS_TOKEN>";
        match service.sign_in_with_idp(Provider::Google, access_token).await {
            Ok(response) => {
                println!("{:?}", response);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}
