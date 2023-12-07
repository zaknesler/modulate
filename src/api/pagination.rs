use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaginatedResponse<T> {
    pub limit: u32,
    pub offset: u32,
    pub total: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub items: Vec<T>,
}
