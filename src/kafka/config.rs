use config::Environment;
use rdkafka::ClientConfig;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct KafkaConfig {
    pub group_id: String,
    pub topic: String,
    pub brokers: String,
    pub keystore_path: Option<String>,
    pub keystore_password: Option<String>,
    pub ca_path: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub partitions: i32,
    pub offset_reset: String,
}

impl KafkaConfig {
    pub fn new() -> Self {
        let builder = config::Config::builder().add_source(Environment::default());

        if std::path::Path::new("Settings.toml").exists() {
            let builder = builder.add_source(config::File::with_name("./Settings.toml"));
            builder
                .build()
                .unwrap()
                .try_deserialize::<KafkaConfig>()
                .unwrap()
        } else {
            builder
                .build()
                .unwrap()
                .try_deserialize::<KafkaConfig>()
                .unwrap()
        }
    }
}

impl Default for KafkaConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl From<KafkaConfig> for ClientConfig {
    fn from(config: KafkaConfig) -> Self {
        let mut client_config = ClientConfig::new();
        client_config
            .set("group.id", config.group_id)
            .set("enable.auto.commit", "true")
            .set("auto.commit.interval.ms", "5000")
            .set("enable.auto.offset.store", "false")
            .set("auto.offset.reset", config.offset_reset)
            .set("bootstrap.servers", config.brokers);

        if let (Some(k_path), Some(k_pass), Some(username), Some(password), Some(ca_path)) = (
            config.keystore_path,
            config.keystore_password,
            config.username,
            config.password,
            config.ca_path,
        ) {
            client_config
                .set("security.protocol", "SASL_SSL")
                .set("ssl.keystore.location", k_path)
                .set("ssl.keystore.password", k_pass)
                .set("ssl.ca.location", ca_path)
                .set("sasl.mechanism", "SCRAM-SHA-512")
                .set("sasl.username", username)
                .set("sasl.password", password);
        } else {
            client_config.set("security.protocol", "PLAINTEXT");
        };

        client_config
    }
}
