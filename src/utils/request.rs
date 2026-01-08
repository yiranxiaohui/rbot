use reqwest::Client;
use tokio::sync::OnceCell;
use crate::config::get_config_clone;

static CLIENT_CELL: OnceCell<Client> = OnceCell::const_new();

pub async fn get_client() -> &'static Client {
    CLIENT_CELL.get_or_init(|| async {
        let config = get_config_clone().await;
        if config.proxy.enabled {
            let proxy = config.proxy;
            let mut scheme = format!("{}://{}:{}", proxy.kind, proxy.address, proxy.port);
            if proxy.username.is_some() && proxy.password.is_some() {
                scheme = format!("{}://{}:{}@{}:{}", proxy.kind, proxy.username.unwrap(), proxy.password.unwrap(), proxy.address, proxy.port);
            }
            let proxy = reqwest::Proxy::all(scheme).unwrap();
            Client::builder()
                .proxy(proxy)
                .build()
                .unwrap()
        } else {
            Client::new()
        }

    }).await
}