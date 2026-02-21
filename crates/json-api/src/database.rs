//! Database connection management

use sqlx::PgPool;

pub(crate) const SET_TENANT_CONTEXT_SQL: &str =
    "SELECT set_config('app.current_tenant_uuid', $1, true)";

/// Connect to `PostgreSQL`.
///
/// # Errors
///
/// Returns an error if the connection cannot be established.
pub(crate) async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(database_url).await
}
