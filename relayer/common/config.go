package common

type PubSubConfig struct {
	SubID                   string `json:"sub_id"`
	TopicID                 string `json:"topic_id"`
	PubsubClientIDFilter    string `json:"pubsub_client_id_filter"`
	PubsubAckDeadlineTime   string `json:"pubsub_ack_deadline_time"`
	PubsubRetentionDuration string `json:"pubsub_retention_duration"`
}

type Config struct {
	ServiceName     string         `json:"service_name"`
	MaxTry          int            `json:"max_try"`
	TxWaitingPeriod string         `json:"tx_waiting_period"`
	NonceInterval   string         `json:"nonce_interval"`
	RpcEndpoints    []string       `json:"rpc_endpoints"`
	Senders         []string       `json:"senders"`
	Contract        string         `json:"contract"`
	SignerUrl       string         `json:"signer_url"`
	ProjectID       string         `json:"project_id"`
	LogLevel        string         `json:"log_level"`
	Subs            []PubSubConfig `json:"subs"`
	Network         string         `json:"network"`
}
