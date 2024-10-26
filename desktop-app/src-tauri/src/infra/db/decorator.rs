use crate::errors::AppError;

use super::pool::{ConnType, PoolType};


// A decorator that applies logic and commits the results if they succeed
pub fn with_transaction<T>(pool: &PoolType, do_fn: impl FnOnce (&rusqlite::Transaction) -> Result<T, AppError>) -> Result<T, AppError> {
    
    let mut conn = pool.get()?;
    let tx = conn.transaction()?;

    let res = do_fn(&tx)?;

    tx.commit()?;

    Ok(res)
}

// A decorator that applies logic and commits the results if they succeed
pub fn with_conn<T>(pool: &PoolType, do_fn: impl FnOnce (&ConnType) -> Result<T, AppError>) -> Result<T, AppError> {
    
    let conn = pool.get()?;
    let res = do_fn(&conn)?;

    Ok(res)
}