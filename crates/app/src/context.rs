//! App Context

use std::sync::Arc;

use thiserror::Error;

use crate::{
    auth::{AuthRepository, PgAuthRepository},
    database,
    products::{PgProductsRepository, ProductsRepository},
};

#[derive(Debug, Error)]
pub enum AppInitError {
    #[error("failed to connect to database")]
    Database(#[source] sqlx::Error),
}

#[derive(Clone)]
pub struct AppContext {
    pub products: Arc<dyn ProductsRepository>,
    pub auth: Arc<dyn AuthRepository>,
}

impl AppContext {
    #[must_use]
    pub fn new(products: Arc<dyn ProductsRepository>, auth: Arc<dyn AuthRepository>) -> Self {
        Self { products, auth }
    }

    /// Build application context from a database URL.
    ///
    /// # Errors
    ///
    /// Returns an error when establishing a database connection fails.
    pub async fn from_database_url(url: &str) -> Result<Self, AppInitError> {
        let pool = database::connect(url)
            .await
            .map_err(AppInitError::Database)?;

        Ok(Self::new(
            Arc::new(PgProductsRepository::new(pool.clone())),
            Arc::new(PgAuthRepository::new(pool)),
        ))
    }
}
