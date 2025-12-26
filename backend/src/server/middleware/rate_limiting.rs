use governor::middleware::NoOpMiddleware;
use tower_governor::{
    governor::{GovernorConfig, GovernorConfigBuilder},
    key_extractor::SmartIpKeyExtractor,
};

pub fn json_limit() -> Box<GovernorConfig<SmartIpKeyExtractor, NoOpMiddleware>> {
    Box::new(
        GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .per_second(20)
            .burst_size(200)
            .finish()
            .unwrap(),
    )
}

pub fn pdf_limit() -> Box<GovernorConfig<SmartIpKeyExtractor, NoOpMiddleware>> {
    Box::new(
        GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .per_second(2)
            .burst_size(4)
            .finish()
            .unwrap(),
    )
}

pub fn auth_limit() -> Box<GovernorConfig<SmartIpKeyExtractor, NoOpMiddleware>> {
    Box::new(
        GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .per_second(2)
            .burst_size(3)
            .finish()
            .unwrap(),
    )
}
