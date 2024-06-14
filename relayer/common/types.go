package common

import (
	"encoding/json"
	"fmt"
	"strconv"

	"cloud.google.com/go/pubsub"
)

type Task struct {
	MsgPubSub    *pubsub.Message
	PriceData    PriceData
	RetryCounter int
}

func RetryTask(ch chan<- Task, task Task, maxTry int, errMsg string) error {
	task.RetryCounter++
	if task.RetryCounter >= maxTry {
		return fmt.Errorf("Task with ReqID-%d reached max retry: %s", task.PriceData.RequestId, errMsg)
	}
	ch <- task
	return nil
}

type SignerPriceDataTS struct {
	Prices      []Price `json:"prices"`
	ResolveTime uint64  `json:"resolveTime"`
	RequestId   uint64  `json:"requestId"`
}

func NewSignerPriceDataTS(spd PriceData) SignerPriceDataTS {
	return SignerPriceDataTS{
		Prices:      spd.Prices,
		ResolveTime: spd.ResolveTime,
		RequestId:   spd.RequestId,
	}
}

type PriceData struct {
	Prices      []Price `json:"prices"`
	ResolveTime uint64  `json:"resolve_time"`
	RequestId   uint64  `json:"request_id"`
}

type Price struct {
	Symbol string    `json:"symbol"`
	Rate   UInt64Str `json:"rate"`
}

type UInt64Str uint64

func (i UInt64Str) MarshalJSON() ([]byte, error) {
	return json.Marshal(strconv.FormatUint(uint64(i), 10))
}

func (i *UInt64Str) UnmarshalJSON(b []byte) error {
	// Try string first
	var s string
	if err := json.Unmarshal(b, &s); err == nil {
		value, err := strconv.ParseUint(s, 10, 64)
		if err != nil {
			return err
		}
		*i = UInt64Str(value)
		return nil
	}

	// Fallback to number
	return json.Unmarshal(b, (*uint64)(i))
}

func (i UInt64Str) UInt64() uint64 {
	return uint64(i)
}

func NewUInt64Str(i uint64) UInt64Str {
	return UInt64Str(i)
}

type Calldata struct {
	Symbols            []string
	MinimumSourceCount uint8
}

func (d Calldata) GetSymbols() []string {
	return d.Symbols
}

func (d Calldata) GetMultiplier() uint64 {
	return 1000000000
}

type Result struct {
	Responses []Response
}

type Response struct {
	Symbol       string
	ResponseCode uint8
	Rate         uint64
}

func (r Result) GetPrices(_ []string) ([]Response, []Response, error) {
	valids := make([]Response, 0)
	fails := make([]Response, 0)
	for _, r := range r.Responses {
		// Check if response code is non-zero, if it is, add to failedResponses and ignore
		if r.ResponseCode == 0 {
			valids = append(valids, r)
		} else {
			fails = append(fails, r)
		}
	}

	return valids, fails, nil
}
