use axerr_derive::AxErr;

#[derive(AxErr)]
pub enum ErrorType {
    #[axerr(status_code = 200, public_message = "whoop")]
    Foo,
    Bar,
    Baz,
}

#[derive(AxErr)]
pub struct Foobar {}
