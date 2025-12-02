
mod raw_client_builder;

pub use raw_client_builder::*;

pub type Client = conjure_runtime::Client<conjure_runtime_local::raw::DefaultRawClient>;

pub type ResponseBody = conjure_runtime::ResponseBody<conjure_runtime_local::raw::DefaultRawBody>;
