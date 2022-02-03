pub mod todo;

#[get("/")]
pub fn hello_world() -> &'static str {
    "hello world"
}
