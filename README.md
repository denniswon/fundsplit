# 🚀 ETH Airdrop Script on Base Mainnet

This is a Zsh-based automation script that uses [Foundry's `cast`](https://book.getfoundry.sh/reference/cast/cast-send) to batch-send **0.25 ETH** from a main wallet to a list of recipient addresses on **Base Mainnet**.

## 🔧 Features

- ✅ Send ETH to multiple addresses from a private key
- ✅ Reads recipient list from a file (`recipients.txt`)
- ✅ Sources environment variables from `.env`
- ✅ Displays transaction hashes and final balances of recipients
- ✅ Supports Base Mainnet via custom RPC

---

## 📦 Requirements

- [Foundry](https://book.getfoundry.sh/getting-started/installation) installed (`cast`)
- Zsh shell
- Base Mainnet wallet with sufficient ETH

---

## 🛠 Setup

1. **Install Foundry** (if not already):

```sh
   curl -L https://foundry.paradigm.xyz | bash
   foundryup
```

2. Create your .env file:

```sh
MAIN_PRIVATE_KEY=your_main_wallet_private_key
ETH_RPC_URL=https://mainnet.base.org
```

3. Add recipient addresses in recipients.txt:

```sh
0xabc123...
0xdef456...
0x789abc...
```

⸻

🚀 Run the Script

```sh
chmod +x fund_workers.zsh
./fund_workers.zsh
```

⸻

✅ Output

For each recipient, the script will:
- Send 0.25 ETH
- Print the transaction hash
- Fetch and display the final balance of the account

⸻

🧠 Notes
- Transactions use the legacy type (for gas compatibility).
- Add a delay between transactions to avoid nonce issues.
- You can customize amount, gas, or RPC by modifying the script.

⸻

🛡️ Security Warning

⚠️ Do not commit your .env file or expose your private key. Use environment vaults or secrets management for production use.
