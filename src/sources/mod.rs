
mod wasabi;

use crate::Result;
use crate::request::{BuildRequest, PreparedRequest};
use crate::response::Response;
pub use crate::sources::wasabi::Wasabi;

pub trait Source {
    fn name(&self) -> &str;
    fn build_request(&self, req: BuildRequest) -> Result<PreparedRequest>;
    fn parse_response(&self, res: &[u8]) -> Result<Response>;
}
