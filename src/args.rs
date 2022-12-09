use clap::Parser;

/// Bulk CSV importer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Service address
    #[arg(short, long, default_value_t = default_address())]
    pub address: String,

    /// Binding user DN
    #[arg(short, long, default_value_t = default_user())]
    pub user: String,

    /// Binding user password
    #[arg(short, long, default_value_t = default_password())]
    pub password: String,

    /// CSV file
    #[arg(short, long, default_value_t = default_csv())]
    pub csv: String,
}

fn default_address() -> String {
    "ldap://192.168.1.25:389".to_string()
}

fn default_user() -> String {
    "cn=svc_ldap,cn=Users,dc=BlackSky,dc=local".to_string()
}

fn default_password() -> String {
    "Admin123".to_string()
}

fn default_csv() -> String {
    "bulk.csv".to_string()
}