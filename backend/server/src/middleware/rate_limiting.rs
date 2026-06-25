use std::sync::Arc;

use governor::{clock::QuantaInstant, middleware::NoOpMiddleware};
use tower_governor::{
    governor::{GovernorConfig, GovernorConfigBuilder},
    key_extractor::SmartIpKeyExtractor,
};

#[derive(Clone)]
pub struct AuthLimit {
    pub arc: Arc<GovernorConfig<SmartIpKeyExtractor, NoOpMiddleware<QuantaInstant>>>,
}

impl Default for AuthLimit {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthLimit {
    pub fn new() -> Self {
        Self {
            arc: Arc::new(
                GovernorConfigBuilder::default()
                    .key_extractor(SmartIpKeyExtractor)
                    .per_second(4)
                    .burst_size(3)
                    .finish()
                    .unwrap(),
            ),
        }
    }
}
pub fn json_limit() -> Box<GovernorConfig<SmartIpKeyExtractor, NoOpMiddleware>> {
    Box::new(
        GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .per_millisecond(100)
            .burst_size(50)
            .finish()
            .unwrap(),
    )
}

pub fn pdf_limit() -> Box<GovernorConfig<SmartIpKeyExtractor, NoOpMiddleware>> {
    Box::new(
        GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .per_millisecond(500)
            .burst_size(3)
            .finish()
            .unwrap(),
    )
}
