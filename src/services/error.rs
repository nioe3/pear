use super::provider_api::error::ProviderApiError;
#[derive(thiserror::Error, Debug)]
pub enum ServiceError {

        #[error("Provider Error: {0}")]
        ProviderApiError(ProviderApiError),
}