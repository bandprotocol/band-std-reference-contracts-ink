package client

import (
	"github.com/bandprotocol/band-std-reference-contracts-ink/relayer/common"
)

type Payload struct {
	PriceData common.SignerPriceDataTS `json:"priceData"`
	From      string                   `json:"from"`
	Nonce     uint64                   `json:"nonce"`
	Tip       uint64                   `json:"tip"`
}

type Result struct {
	Tx string `json:"tx"`
}
