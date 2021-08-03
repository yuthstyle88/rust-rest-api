use actix_web::dev::RequestHead;
pub fn verify_api_key(req: &RequestHead) -> bool {
  /*  let key = get_key_api(req);
    if let Ok(api_key) = key {
        let is_ok = blocking(context.pool(), move |conn| {
            ClientApps::find_by_api_key(conn, api_key)
        }).await;
        match is_ok {
            Ok(Ok(res)) => res,
            Err(err) => {
                log::info!("* Verify api key false! => {:?}",err);
                false
            }
            _ => false
        };
    }*/
    false
}