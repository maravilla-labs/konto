use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct PaginationParams {
    #[param(default = 1, minimum = 1)]
    pub page: Option<u64>,
    #[param(default = 50, minimum = 1, maximum = 200)]
    pub per_page: Option<u64>,
    pub search: Option<String>,
    pub format: Option<String>,
    pub category: Option<String>,
}

impl PaginationParams {
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn per_page(&self) -> u64 {
        self.per_page.unwrap_or(50).clamp(1, 200)
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: u64, page: u64, per_page: u64) -> Self {
        let total_pages = if per_page > 0 {
            total.div_ceil(per_page)
        } else {
            0
        };
        Self {
            data,
            total,
            page,
            per_page,
            total_pages,
        }
    }
}
