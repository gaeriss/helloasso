pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Env(#[from] envir::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("{0}")]
    Regex(#[from] regex::Error),
    #[error("{0}")]
    Server(#[from] attohttpc::Error),
    #[error("Invalid selector")]
    Selector,
}

impl From<&Error> for actix_web::http::StatusCode {
    fn from(_: &Error) -> Self {
        Self::INTERNAL_SERVER_ERROR
    }
}

impl actix_web::error::ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse {
        let status = actix_web::http::StatusCode::from(self);

        if status.is_client_error() {
            log::warn!("{self:?}");
        } else if status.is_server_error() {
            log::error!("{self:?}");
        }

        actix_web::HttpResponse::build(status).json(serde_json::json!({
            "error": self.to_string(),
        }))
    }
}
