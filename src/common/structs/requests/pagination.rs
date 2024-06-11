use serde::Deserialize;
use utoipa::IntoParams;


pub const MIN_LIMIT: u64 = 20;
pub const MAX_LIMIT: u64 = 200;

#[derive(Deserialize, IntoParams)]
pub struct Pagination {
    #[param(nullable = true, example = 1, default = 1)]
    pub page: Option<u64>,
    #[param(nullable = true, example = 20, default = 20, minimum = 20, maximum = 200)]
    pub limit: Option<u64>,
}

impl Pagination {
    pub fn calculate_offset_and_limit(&self) -> (u64, u64) {
        let limit = self.limit.unwrap_or(MIN_LIMIT);
        let page = self.page.unwrap_or(1);

        let limit = if 0 < limit && limit <= MAX_LIMIT { limit } else { MIN_LIMIT };
        let offset = if page > 0 { (page - 1) * limit } else { 0 };

        (offset, limit)
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(MIN_LIMIT),
        }
    }
}
