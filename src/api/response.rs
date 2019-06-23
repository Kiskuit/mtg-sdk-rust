use reqwest::header::HeaderMap;

/// Response returned by the Cards API
#[allow(dead_code)]
pub struct ApiResponse<T> {
    pub content: T,
    pub page_size: Option<u32>,
    pub count: Option<u32>,
    pub total_count: Option<u32>,
    pub ratelimit_limit: Option<u32>,
    pub ratelimit_remaining: Option<u32>,
}

impl<T> ApiResponse<T> {
    pub(crate) fn new(content: T, headers: &HeaderMap) -> ApiResponse<T> {
        let page_size = headers.get("Page-Size")
            .unwrap().to_str().unwrap().parse::<u32>().unwrap();
        let count = headers.get("Count")
            .unwrap().to_str().unwrap().parse::<u32>().unwrap();
        let total_count = headers.get("Total-Count")
            .unwrap().to_str().unwrap().parse::<u32>().unwrap();
        let ratelimit_limit = headers.get("Ratelimit-Limit")
            .unwrap().to_str().unwrap().parse::<u32>().unwrap();
        let ratelimit_remaining = headers.get("Ratelimit-Remaining")
            .unwrap().to_str().unwrap().parse::<u32>().unwrap();
        let page_size = Some(page_size);
        let count = Some(count);
        let total_count = Some(total_count);
        let ratelimit_limit = Some(ratelimit_limit);
        let ratelimit_remaining = Some(ratelimit_remaining);
        ApiResponse {
            content,
            page_size,
            count,
            total_count,
            ratelimit_limit,
            ratelimit_remaining,
        }
    }
}
