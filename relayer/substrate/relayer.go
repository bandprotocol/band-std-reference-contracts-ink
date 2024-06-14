package substrate

import (
	"time"

	"github.com/bandprotocol/band-std-reference-contracts-ink/relayer/common"
	"github.com/bandprotocol/band-std-reference-contracts-ink/relayer/substrate/client"
	log "github.com/sirupsen/logrus"
)

type Relayer struct {
	PendingTasks chan common.Task
	FreeSenders  chan string

	Client        client.Client
	logger        *log.Logger
	SignerUrl     string
	Tip           uint64
	NonceInterval time.Duration
	MaxTry        int
}

func NewRelayer(
	client client.Client,
	pendingTasks chan common.Task,
	logger *log.Logger,
	senders []string,
	signerUrl string,
	tip uint64,
	nonceInterval time.Duration,
	maxTry int,
) *Relayer {
	r := Relayer{
		PendingTasks:  pendingTasks,
		FreeSenders:   make(chan string, len(senders)),
		Client:        client,
		logger:        logger,
		SignerUrl:     signerUrl,
		Tip:           tip,
		NonceInterval: nonceInterval,
		MaxTry:        maxTry,
	}

	for _, sender := range senders {
		r.FreeSenders <- sender
	}

	return &r
}

func (r Relayer) Start() {
	for {
		task := <-r.PendingTasks
		sender := <-r.FreeSenders
		go r.HandleRelay(task, sender)
	}
}

func (r Relayer) HandleRelay(task common.Task, sender string) {
	defer func() {
		r.FreeSenders <- sender
	}()

	nonce, err := r.Client.GetNonce(sender)
	if err != nil {
		common.RetryTask(r.PendingTasks, task, r.MaxTry, err.Error())
		return
	}

	signerPriceData := common.NewSignerPriceDataTS(task.PriceData)
	signedTx, err := r.Client.SignExtrinsic(signerPriceData, sender, nonce, r.Tip)
	if err != nil {
		common.RetryTask(r.PendingTasks, task, r.MaxTry, err.Error())
		return
	}

	_, err = r.Client.BroadcastExtrinsic(signedTx)
	if err != nil {
		common.RetryTask(r.PendingTasks, task, r.MaxTry, err.Error())
		return
	}

	if task.MsgPubSub != nil {
		task.MsgPubSub.Ack()
	}
}
