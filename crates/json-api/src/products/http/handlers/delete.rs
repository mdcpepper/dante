//! Delete Product Handler

use std::sync::Arc;

use salvo::{oapi::extract::PathParam, prelude::*};
use uuid::Uuid;

use crate::{extensions::*, state::State};

/// Delete Product Handler
#[endpoint(
    tags("products"),
    summary = "Delete Product",
    responses(
        (status_code = StatusCode::OK, description = "Product deleted"),
        (status_code = StatusCode::NOT_FOUND, description = "Product not found"),
        (status_code = StatusCode::BAD_REQUEST, description = "Bad Request"),
        (status_code = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
    ),
)]
pub(crate) async fn handler(
    uuid: PathParam<Uuid>,
    depot: &mut Depot,
) -> Result<StatusCode, StatusError> {
    depot
        .obtain_or_500::<Arc<State>>()?
        .products
        .delete_product(uuid.into_inner())
        .await
        .map_err(StatusError::from)?;

    Ok(StatusCode::OK)
}

#[cfg(test)]
mod tests {
    use salvo::{affix_state::inject, test::TestClient};
    use testresult::TestResult;

    use crate::products::{MockProductsRepository, ProductsRepositoryError};

    use super::{super::tests::*, *};

    fn make_service(repo: MockProductsRepository) -> Service {
        let state = Arc::new(State::new(Arc::new(repo)));

        let router = Router::new()
            .hoop(inject(state))
            .push(Router::with_path("products/{uuid}").delete(handler));

        Service::new(router)
    }

    #[tokio::test]
    async fn test_delete_product_success() -> TestResult {
        let uuid = Uuid::now_v7();

        make_product(uuid);

        let mut repo = MockProductsRepository::new();

        repo.expect_delete_product()
            .once()
            .withf(move |u| *u == uuid)
            .return_once(move |_| Ok(()));

        let res = TestClient::delete(format!("http://example.com/products/{uuid}"))
            .send(&make_service(repo))
            .await;

        assert_eq!(res.status_code, Some(StatusCode::OK));

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_product_invalid_uuid_returns_400() -> TestResult {
        let res = TestClient::delete("http://example.com/products/123")
            .send(&make_service(MockProductsRepository::new()))
            .await;

        assert_eq!(res.status_code, Some(StatusCode::BAD_REQUEST));

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_product_not_found_returns_404() -> TestResult {
        let uuid = Uuid::now_v7();

        let mut repo = MockProductsRepository::new();

        repo.expect_delete_product()
            .once()
            .withf(move |u| *u == uuid)
            .return_once(|_| Err(ProductsRepositoryError::InvalidReference));

        repo.expect_create_product().never();
        repo.expect_get_products().never();
        repo.expect_update_product().never();

        let res = TestClient::delete(format!("http://example.com/products/{uuid}"))
            .send(&make_service(repo))
            .await;

        assert_eq!(res.status_code, Some(StatusCode::BAD_REQUEST));

        Ok(())
    }
}
