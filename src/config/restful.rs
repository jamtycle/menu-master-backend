use rocket::{
    fairing::{Fairing, Info, Kind},
    Request, Response, http::ContentType,
};

pub struct RESTFul;

#[async_trait]
impl Fairing for RESTFul {
    fn info(&self) -> Info {
        Info {
            name: "Add application/json mime type to all responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, _res: &mut Response<'r>) {
        _res.set_header(ContentType::JSON);
    }
}
