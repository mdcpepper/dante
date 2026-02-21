//! Errors

use salvo::http::StatusError;
use tracing::error;

use lattice_app::products::ProductsRepositoryError;

pub(crate) fn into_status_error(error: ProductsRepositoryError) -> StatusError {
    match error {
        ProductsRepositoryError::AlreadyExists => {
            StatusError::conflict().brief("Product already exists")
        }
        ProductsRepositoryError::InvalidPrice(_) => {
            StatusError::bad_request().brief("Price is out of range")
        }
        ProductsRepositoryError::InvalidReference
        | ProductsRepositoryError::MissingRequiredData
        | ProductsRepositoryError::InvalidData => {
            StatusError::bad_request().brief("Invalid product payload")
        }
        ProductsRepositoryError::Sql(source) => {
            error!("failed to create product: {source}");

            StatusError::internal_server_error()
        }
        ProductsRepositoryError::NotFound => {
            error!("product not found");

            StatusError::not_found()
        }
    }
}
