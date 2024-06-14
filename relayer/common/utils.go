package common

import (
	"time"
)

func MustParseDuration(d string) time.Duration {
	duration, err := time.ParseDuration(d)
	if err != nil {
		panic(err)
	}
	return duration
}
