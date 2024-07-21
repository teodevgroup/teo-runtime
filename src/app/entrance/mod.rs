#[derive(Copy, Clone, Debug)]
pub enum Entrance {
    APP,
    CLI,
}

impl Entrance {

    pub fn to_str(&self) -> &'static str {
        match self {
            Entrance::APP => "APP",
            Entrance::CLI => "CLI",
        }
    }
}
