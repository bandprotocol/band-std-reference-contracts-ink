package client

import (
	"time"

	"github.com/go-resty/resty/v2"
	log "github.com/sirupsen/logrus"

	"github.com/bandprotocol/band-std-reference-contracts-ink/relayer/common"
)

var _ Client = ContractClient{}

type ContractClient struct {
	BaseClient

	contractAddr  string
	resultPointer string
}

// NewContractClient creates a new instance of ContractClient with the provided parameters.
func NewContractClient(
	logger *log.Logger,
	rpcEndpoints []string,
	txWaitingPeriod time.Duration,
	signerURL string,
	contractAddr string,
	resultPointer string,
) ContractClient {
	return ContractClient{
		BaseClient:    NewBaseClient(logger, rpcEndpoints, txWaitingPeriod, signerURL),
		contractAddr:  contractAddr,
		resultPointer: resultPointer,
	}
}

// SignExtrinsic signs an extrinsic using the specified parameters.
func (c ContractClient) SignExtrinsic(
	prices common.SignerPriceDataTS,
	relayer string,
	nonce uint64,
	tip uint64,
) (string, error) {
	payload := Payload{
		PriceData: prices,
		From:      relayer,
		Nonce:     nonce,
		Tip:       tip,
	}

	var result Result
	client := resty.New()

	_, err := client.R().SetBody(payload).SetResult(&result).Post(c.SignerURL)
	if err != nil {
		return "", err
	}

	return result.Tx, nil
}
