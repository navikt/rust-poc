use crate::Blueprint;

pub fn blueprint() -> Blueprint {
    Blueprint {
        basepath: "/",
        paths: routes![hello]
    }
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}