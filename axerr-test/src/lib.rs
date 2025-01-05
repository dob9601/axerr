use axerr_derive::AxErr;

#[derive(AxErr)]
pub enum ErrorType {
    Foo,
    Bar,
    Baz,
}
