use rocket::Route;

pub struct Blueprint {
    pub basepath: &'static str,
    pub paths: Vec<Route>
}