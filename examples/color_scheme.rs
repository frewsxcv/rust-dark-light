#[tokio::main]
async fn main() {
    let connection = ashpd::zbus::Connection::session().await.unwrap();
    let proxy = ashpd::desktop::settings::SettingsProxy::new(&connection).await.unwrap();
    let color_scheme = proxy.color_scheme().await.unwrap();
    match color_scheme {
        ashpd::desktop::settings::ColorScheme::PreferDark => println!("Dark"),
        ashpd::desktop::settings::ColorScheme::PreferLight => println!("Light"),
        ashpd::desktop::settings::ColorScheme::NoPreference => println!("Default"),
    };
}