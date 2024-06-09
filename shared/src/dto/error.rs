use std::error::Error;

pub type BoxDynError = Box<dyn Error + Sync + Send + 'static>;
