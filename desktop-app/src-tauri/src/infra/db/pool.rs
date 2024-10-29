use r2d2_sqlite::SqliteConnectionManager;

pub type PoolType = r2d2::Pool<SqliteConnectionManager>;

pub type ConnType = r2d2::PooledConnection<SqliteConnectionManager>;
