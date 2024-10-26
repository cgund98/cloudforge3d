use std::future::Future;

use crate::errors::AppError;

use super::pool::{ConnType, PoolType};

// A decorator that applies logic and commits the results if they succeed
pub fn with_transaction<T>(
    pool: &PoolType,
    do_fn: impl FnOnce(&rusqlite::Transaction) -> Result<T, AppError>,
) -> Result<T, AppError> {
    let mut conn = pool.get()?;
    let tx = conn.transaction()?;

    let res = do_fn(&tx);

    if let Ok(out) = res {
        let _ = tx.commit();
        return Ok(out);
    }

    let _ = tx.rollback();
    let err = res.err().unwrap();

    Err(err)
}

// A decorator that applies logic and commits the results if they succeed (async input)
pub async fn with_async_transaction<Fut, T>(
    pool: &PoolType,
    do_fn: impl FnOnce(&rusqlite::Transaction<'_>) -> Fut,
) -> Result<T, AppError>
where
    Fut: Future<Output = Result<T, AppError>> + '_,
{
    let mut conn = pool.get()?;
    let tx = conn.transaction()?;

    let res = do_fn(&tx).await;

    if let Ok(out) = res {
        let _ = tx.commit();
        return Ok(out);
    }

    let _ = tx.rollback();
    let err = res.err().unwrap();

    Err(err)
}

// A decorator that applies logic and commits the results if they succeed
pub fn with_conn<T>(
    pool: &PoolType,
    do_fn: impl FnOnce(&ConnType) -> Result<T, AppError>,
) -> Result<T, AppError> {
    let conn = pool.get()?;
    let res = do_fn(&conn)?;

    Ok(res)
}
