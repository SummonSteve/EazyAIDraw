
pub struct DrawCallResult {

}

pub trait DrawCallResponse {
    fn from_http_response(&self, res: reqwest::Response) -> DrawCallResult;
}
    