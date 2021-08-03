use note_utils::MyError;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use crate::app::DbPool;

#[allow(dead_code)]
pub async fn blocking<F, T>(pool: &DbPool, f: F) -> Result<T, MyError>
    where
        F: FnOnce(&diesel::PgConnection) -> T + Send + 'static,
        T: Send + 'static,
{
    let pool = pool.clone();
    let res = actix_web::web::block(move || {
        let conn = pool.get()?;
        let res = (f)(&conn);
        Ok(res) as Result<T, MyError>
    })
        .await?;
    res
}
