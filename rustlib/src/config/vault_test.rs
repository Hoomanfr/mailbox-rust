#[cfg(test)]
mod tests{
    use std::env;
    use crate::config::vault::ConfigManager;

    #[tokio::test]
    async fn test_get_value(){
        env::set_var("ENVIRONMENT", "dev");
        env::set_var("DOMAIN", "wms");
        env::set_var("SERVICE", "mailbox");
        env::set_var("VAULT_ADDR", "https://34.168.165.181:8200");
        env::set_var("VAULT_TOKEN", "s.rCO7OV2nfAz10ocGATOJcDDn");
        let cfg = ConfigManager::new();
        let result = cfg.get_value("DATABASE_URL".to_string()).await;
        match result {
            Ok(value) => {
                assert!(value.contains("postgres"));
            },
            Err(_) => {
                assert!(false);
            }
        }
    }
}