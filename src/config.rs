use mysql::Opts;
use mysql::OptsBuilder;

#[derive(Clone, Debug)]
pub struct Config {
    pub user: Option<String>,
    pub pass: Option<String>,
    pub host: String,
    pub port: String
}

impl Into<Opts> for Config {
    fn into(self) -> Opts {
        let mut builder = OptsBuilder::new();
        builder.ip_or_hostname(Some(self.host));
        builder.tcp_port(self.port.parse::<u16>().expect("port is not an u16") as u16);
        builder.user(self.user);
        builder.pass(self.pass);
        builder.prefer_socket(false);
        builder.into()
    }
}