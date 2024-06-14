package substrate

import (
	"encoding/json"
	"os"

	"github.com/bandprotocol/band-std-reference-contracts-ink/relayer/common"
)

type Config struct {
	common.Config

	/*
		ResultPointer can be obtained by following these steps:
		1. Open the website: https://contracts-ui.substrate.io/
		2. Set contract query: Perform a contract query.
		3. Open inspect: Right-click on the page, select "Inspect" or use a keyboard shortcut (Ctrl+Shift+I or Cmd+Option+I).
		4. Navigate to the network tab: Click on the "Network" tab within the developer tools.
		5. Go to the WebSocket (ws) tab: Look for a WebSocket tab within the network tab.
		6. Use getReferenceDataBulk in UI: Execute a query using the getReferenceDataBulk function to interact with the contract.
		7. Check the ws msg: find the response after state call.
		8. Check the msg result: Examine the WebSocket tab to find the data between the contract address and the price data, with
		the format "0X000X00000000000000000000000000000000000000000000" (where 'x' can represent any hexadecimal digit).
	*/
	ResultPointer string `json:"result_pointer"`

	Tip uint64 `json:"tip"`
}

func GetConfig() (*Config, error) {
	// read file as bytes
	rawByte, err := os.ReadFile(os.Getenv("CONFIG_PATH"))
	if err != nil {
		return nil, err
	}

	var cfg Config
	if err := json.Unmarshal(rawByte, &cfg); err != nil {
		return nil, err
	}

	return &cfg, nil
}
