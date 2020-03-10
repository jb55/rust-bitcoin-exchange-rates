
mod wasabi;
mod bitfinex;

use crate::Result;
use crate::data::{BuildRequest, PreparedRequest, Response};
pub use crate::sources::wasabi::Wasabi;
pub use crate::sources::bitfinex::Bitfinex;

pub trait Source {
    fn name(&self) -> &str;
    fn build_request(&self, req: BuildRequest) -> Result<PreparedRequest>;
    fn parse_response(&self, res: &[u8]) -> Result<Response>;
}
