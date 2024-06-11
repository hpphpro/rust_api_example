use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::v1::doc::ApiDoc;

use super::common::middlewares::setup::setup_middlewares;


pub async fn create_general_router(routers: Vec<Router>) -> Router {
    let mut sub_router = Router::new();

    for router in routers {
        sub_router = sub_router.merge(router)
    }

    let mut main_router = Router::new()
        .nest("/api/", sub_router)
        .merge(SwaggerUi::new("/api/docs").url("/api-docs/openapi.json", ApiDoc::openapi()));

    main_router = setup_middlewares(main_router);

    main_router
}