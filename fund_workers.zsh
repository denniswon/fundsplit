#!/bin/zsh

# Load environment variables
source .env

# Validate env vars
if [[ -z "$MAIN_PRIVATE_KEY" || -z "$RPC_URL" ]]; then
  echo "❌ MAIN_PRIVATE_KEY or RPC_URL is not set in .env"
  exit 1
fi

# Config
AMOUNT="0.25ether"
GAS_LIMIT="21000"
RECIPIENT_FILE="recipients.txt"

# Check file exists
if [[ ! -f "$RECIPIENT_FILE" ]]; then
  echo "❌ File $RECIPIENT_FILE not found"
  exit 1
fi

# Read recipient addresses
RECIPIENTS=("${(@f)$(<"$RECIPIENT_FILE")}")

# Track tx hashes
TX_HASHES=()

echo "🚀 Sending $AMOUNT to each recipient on Base mainnet..."

for RECIPIENT in "${RECIPIENTS[@]}"; do
  echo "→ Sending to $RECIPIENT..."

  TX_OUTPUT=$(cast send --private-key "$MAIN_PRIVATE_KEY" \
                        --value "$AMOUNT" \
                        --legacy \
                        --rpc-url "$RPC_URL" \
                        "$RECIPIENT" "" \
                        --gas-limit "$GAS_LIMIT")

  TX_HASH=$(echo "$TX_OUTPUT" | grep -i "Transaction Hash" | awk '{print $NF}')
  
  if [[ -n "$TX_HASH" ]]; then
    echo "   ✅ TX Hash: $TX_HASH"
    TX_HASHES+=("$TX_HASH")
  else
    echo "   ❌ Failed to send to $RECIPIENT"
    TX_HASHES+=("FAILED")
  fi

  sleep 5  # Optional delay between transactions
done

echo "\n📦 Transaction Results:"
for i in "${!RECIPIENTS[@]}"; do
  echo "Recipient: ${RECIPIENTS[$i]}"
  echo "  TxHash: ${TX_HASHES[$i]}"

  BAL_WEI=$(cast balance "${RECIPIENTS[$i]}" --rpc-url "$RPC_URL")
  BAL_ETH=$(cast from-wei "$BAL_WEI")
  echo "  Balance: $BAL_ETH ETH"
done
