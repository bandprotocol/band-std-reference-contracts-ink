package common

import (
	"context"
	"fmt"
	"time"

	"cloud.google.com/go/pubsub"
	log "github.com/sirupsen/logrus"
)

func CreateSub(ps PubSubConfig, projectID string) (*pubsub.Subscription, error) {
	pubsubCtx := context.Background()
	client, err := pubsub.NewClient(pubsubCtx, projectID)
	if err != nil {
		panic(err)
	}

	sub := client.Subscription(ps.SubID)
	exist, err := sub.Exists(pubsubCtx)
	if err != nil {
		panic(err)
	}
	if !exist {
		pubsubDeadlineAckTime := MustParseDuration(ps.PubsubAckDeadlineTime)
		pubsubRetentionTime := MustParseDuration(ps.PubsubRetentionDuration)
		sub, err = client.CreateSubscription(pubsubCtx, ps.SubID, pubsub.SubscriptionConfig{
			Topic: client.Topic(ps.TopicID),
			Filter: fmt.Sprintf(
				`hasPrefix(attributes.ClientID, "%s")`,
				ps.PubsubClientIDFilter,
			),
			AckDeadline:       pubsubDeadlineAckTime,
			RetentionDuration: pubsubRetentionTime,
			ExpirationPolicy:  time.Duration(7 * 24 * time.Hour),
		})
		if err != nil {
			return nil, err
		}
	}

	return sub, nil
}

func SubRequester(
	logger *log.Logger,
	sub *pubsub.Subscription,
	pendingTasks chan<- Task,
) {
	err := sub.Receive(context.Background(), func(pubsubCtx context.Context, msg *pubsub.Message) {
		result, err := ExtractResultFromMsg(msg.Data)
		if err != nil {
			msg.Ack()
			return
		}

		task, err := TaskFromResult(result)
		if err != nil {
			msg.Ack()
			return
		}

		if len(task.PriceData.Prices) > 0 {
			task.MsgPubSub = msg
			pendingTasks <- task
			logger.Infof("Got new task: %+v", task)
		} else {
			msg.Ack()
		}
	})
	if err != nil {
		panic(err)
	}
}
