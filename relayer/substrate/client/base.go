package client

import (
	"context"
	"fmt"
	"time"

	gsrpc "github.com/centrifuge/go-substrate-rpc-client/v4"
	"github.com/centrifuge/go-substrate-rpc-client/v4/config"
	gethrpc "github.com/centrifuge/go-substrate-rpc-client/v4/gethrpc"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
	log "github.com/sirupsen/logrus"
	"github.com/vedhavyas/go-subkey/v2"
)

type BaseClient struct {
	Clients         []gsrpc.SubstrateAPI
	Logger          *log.Logger
	TxWaitingPeriod time.Duration

	SignerURL string
}

func NewBaseClient(
	logger *log.Logger,
	rpcEndpoints []string,
	txWaitingPeriod time.Duration,
	signerURL string,
) BaseClient {
	var clients []gsrpc.SubstrateAPI

	for _, rpcEndpoint := range rpcEndpoints {
		client, err := gsrpc.NewSubstrateAPI(rpcEndpoint)
		if err != nil {
			panic(err)
		}

		clients = append(clients, *client)
	}

	return BaseClient{
		Clients:         clients,
		Logger:          logger,
		TxWaitingPeriod: txWaitingPeriod,
		SignerURL:       signerURL,
	}
}

func (c BaseClient) BroadcastExtrinsic(encodeExt string) (types.Hash, error) {
	ch := make(chan types.ExtrinsicStatus, len(c.Clients))
	passed := false
	var subs []*gethrpc.ClientSubscription
	for _, client := range c.Clients {
		sub, err := SubmitAndWatchRawExtrinsic(&client, encodeExt, ch)
		if err != nil {
			c.Logger.Warningf(
				"Failed to submit extrinsic on %s with error %+v",
				client.Client.URL(),
				err,
			)
			continue
		}
		subs = append(subs, sub)
		passed = true
	}
	defer func() {
		for _, s := range subs {
			s.Unsubscribe()
		}
	}()
	if !passed {
		return types.Hash{}, fmt.Errorf("can't broadcast extrinsic from all endpoints")
	}
	for {
		select {
		case <-time.After(c.TxWaitingPeriod):
			return types.Hash{}, fmt.Errorf("the extrinsic has not been included in a block within the waiting period")
		case status := <-ch:
			if status.IsInBlock {
				blockHash := status.AsInBlock
				c.Logger.Infof("Submit Extrinsic Success in a block %#x", blockHash)
				return blockHash, nil
			}
		}
	}
}

func (c BaseClient) GetNonce(account string) (uint64, error) {
	nonceMac := uint64(0)
	foundNonce := false
	for _, client := range c.Clients {
		accs, err := getAccounts(client, []string{account})
		if err != nil {
			c.Logger.Warningf(
				"Failed to get nonce on %s with error %+v",
				client.Client.URL(),
				err,
			)
			continue
		}
		acc := accs[0]
		nonce := uint64(acc.Nonce)
		foundNonce = true
		if nonceMac < nonce {
			nonceMac = nonce
		}
	}
	if foundNonce {
		return nonceMac, nil
	} else {
		return 0, fmt.Errorf("failed to get nonce from all endpoint")
	}
}

func getAccounts(api gsrpc.SubstrateAPI, accounts []string) ([]types.AccountInfo, error) {
	meta, err := api.RPC.State.GetMetadataLatest()
	if err != nil {
		return []types.AccountInfo{}, err
	}

	accs := []types.AccountInfo{}

	for _, account := range accounts {
		_, publicKey, _ := subkey.SS58Decode(account)

		key, err := types.CreateStorageKey(meta, "System", "Account", publicKey, nil)
		if err != nil {
			return []types.AccountInfo{}, err
		}

		var accountInfo types.AccountInfo
		ok, err := api.RPC.State.GetStorageLatest(key, &accountInfo)
		if err != nil {
			return []types.AccountInfo{}, err
		}
		if !ok {
			return []types.AccountInfo{}, fmt.Errorf("the %s account was not found", account)
		}

		accs = append(accs, accountInfo)
	}

	return accs, nil
}

func SubmitAndWatchRawExtrinsic(
	api *gsrpc.SubstrateAPI,
	enc string,
	c chan<- types.ExtrinsicStatus,
) (*gethrpc.ClientSubscription, error) {
	ctx, cancel := context.WithTimeout(context.Background(), config.Default().SubscribeTimeout)
	defer cancel()

	return api.Client.Subscribe(
		ctx,
		"author",
		"submitAndWatchExtrinsic",
		"unwatchExtrinsic",
		"extrinsicUpdate",
		c,
		enc,
	)
}
