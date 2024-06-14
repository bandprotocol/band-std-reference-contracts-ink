package main

import (
	"github.com/bandprotocol/band-std-reference-contracts-ink/relayer/common"
	"github.com/bandprotocol/band-std-reference-contracts-ink/relayer/substrate"
	"github.com/bandprotocol/band-std-reference-contracts-ink/relayer/substrate/client"
	"github.com/sirupsen/logrus"
)

func main() {
	cfg, err := substrate.GetConfig()
	if err != nil {
		panic(err)
	}

	logger := logrus.New()

	txWaitingPeriod := common.MustParseDuration(cfg.TxWaitingPeriod)
	nonceInterval := common.MustParseDuration(cfg.NonceInterval)

	pendingTasks := make(chan common.Task, 100)
	for _, pubSubConfig := range cfg.Subs {
		sub, err := common.CreateSub(pubSubConfig, cfg.ProjectID)
		if err != nil {
			panic(err)
		}
		go common.SubRequester(
			logger,
			sub,
			pendingTasks,
		)
	}

	var c client.Client
	c = client.NewContractClient(
		logger,
		cfg.RpcEndpoints,
		txWaitingPeriod,
		cfg.SignerUrl,
		cfg.Contract,
		cfg.ResultPointer,
	)

	substrateRelayer := substrate.NewRelayer(
		c,
		pendingTasks,
		logger,
		cfg.Senders,
		cfg.SignerUrl,
		cfg.Tip,
		nonceInterval,
		cfg.MaxTry,
	)

	substrateRelayer.Start()
}
