use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaginatedResponse<T> {
    pub limit: u32,
    pub offset: u32,
    pub total: u32,
    pub items: Vec<T>,
    pub href: String,
    pub previous: Option<String>,
    pub next: Option<String>,
}
