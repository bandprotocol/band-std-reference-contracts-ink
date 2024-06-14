package common

import (
	"encoding/json"

	"github.com/bandprotocol/bandchain-packet/obi"
	oracletypes "github.com/bandprotocol/bandchain-packet/packet"
)

func ExtractResultFromMsg(jsonBytes []byte) (oracletypes.Result, error) {
	var r oracletypes.Result
	if err := json.Unmarshal(jsonBytes, &r); err != nil {
		return oracletypes.Result{}, err
	}

	return r, nil
}

func TaskFromResult(r oracletypes.Result) (Task, error) {
	var cd Calldata
	err := obi.Decode(r.Calldata, &cd)
	if err != nil {
		return Task{}, err
	}

	var decoded Result
	err = obi.Decode(r.Result, &decoded)
	if err != nil {
		return Task{}, err
	}

	prices, _, err := decoded.GetPrices(cd.GetSymbols())
	if err != nil {
		return Task{}, err
	}

	priceData := PriceData{
		ResolveTime: uint64(r.ResolveTime),
		RequestId:   r.RequestID,
	}

	for _, price := range prices {
		priceData.Prices = append(
			priceData.Prices, Price{
				Symbol: price.Symbol,
				Rate:   UInt64Str(price.Rate),
			},
		)
	}

	return Task{
		PriceData:    priceData,
		RetryCounter: 0,
	}, nil
}
