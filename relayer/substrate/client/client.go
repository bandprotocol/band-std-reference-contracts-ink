package client

import (
	"github.com/bandprotocol/band-std-reference-contracts-ink/relayer/common"
	"github.com/centrifuge/go-substrate-rpc-client/v4/types"
)

type Client interface {
	GetNonce(account string) (uint64, error)
	BroadcastExtrinsic(signedExt string) (types.Hash, error)
	SignExtrinsic(
		prices common.SignerPriceDataTS,
		relayer string,
		nonce uint64,
		tip uint64,
	) (string, error)
}
