use crate::Blueprint;

pub fn blueprint() -> Blueprint {
    Blueprint {
        basepath: "/internal",
        paths: routes![is_alive, is_ready]
    }
}

#[get("/isAlive")]
fn is_alive() -> &'static str {
    "I'm alive!"
}

#[get("/isReady")]
fn is_ready() -> &'static str {
    "I'm ready!"
}
