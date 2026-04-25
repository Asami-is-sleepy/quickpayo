# QuickPayo

Instant freelancer payments on Stellar — no banks, no 5-day waits, near-zero fees.

## The Problem

A freelance video editor in Quezon City finishes a project for an overseas client. They send a PayPal invoice and wait. 5–10 days later, ₱5,000 arrives — minus ₱300 in fees. Rent is due tomorrow.

**QuickPayo fixes this.** The client pays in USDC via Stellar. A Soroban smart contract confirms it on-chain. The freelancer gets the money in seconds.

---

## How It Works

```
Freelancer creates invoice  →  Client pays in USDC  →  Contract confirms on-chain  →  Funds released instantly
```

| | PayPal | QuickPayo |
|---|---|---|
| Settlement time | 5–10 days | ~5 seconds |
| Fees | ~6% | ~$0.00001 |
| Requires bank account | Yes | No |
| Works cross-border | Slow | Native |

---

## Stellar Features Used

- **USDC transfers** — payments in a stable, dollar-pegged asset
- **Soroban smart contracts** — trustless invoice state management on-chain
- **Trustlines** — ensures the freelancer's account can receive USDC before payment

---

## Contract Functions

### `create_invoice`
Creates a new invoice stored on-chain.

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --fn create_invoice \
  -- \
  --invoice_id 0101010101010101010101010101010101010101010101010101010101010101 \
  --client <CLIENT_ADDRESS> \
  --freelancer <FREELANCER_ADDRESS> \
  --amount 5000
```

### `pay_invoice`
Marks an invoice as paid. Panics if already paid or not found.

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --fn pay_invoice \
  -- \
  --invoice_id 0101010101010101010101010101010101010101010101010101010101010101
```

### `check_status`
Returns `true` if the invoice has been paid.

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --fn check_status \
  -- \
  --invoice_id 0101010101010101010101010101010101010101010101010101010101010101
```

---

## Error Codes

| Code | Meaning |
|---|---|
| `InvoiceExists` (1) | An invoice with this ID already exists |
| `NotFound` (2) | No invoice found for this ID |
| `AlreadyPaid` (3) | Invoice has already been paid |

---

## Prerequisites

- [Rust](https://rustup.rs/) with `wasm32v1-none` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/stellar-cli)

```bash
rustup target add wasm32v1-none
```

---

## Build & Test

```bash
# Build the contract
stellar contract build --manifest-path contracts/quickpayo/Cargo.toml

# Run tests
cargo test --manifest-path contracts/quickpayo/Cargo.toml
```

---

## Deploy to Testnet

```bash
stellar contract deploy \
  --wasm contracts/quickpayo/target/wasm32v1-none/release/quickpayo.wasm \
  --source <YOUR_SECRET_KEY> \
  --network testnet
```

---

## Project Structure

```
contracts/quickpayo/
├── Cargo.toml
└── src/
    ├── lib.rs      # Contract logic
    └── test.rs     # 8 tests covering happy path + all error cases
```

---

## Target Users

Freelancers (video editors, designers, developers) in the Philippines and Southeast Asia earning ₱10k–₱50k/month from overseas clients — anyone who has lost money and time waiting for international payments to clear.

---

## License

MIT

https://stellar.expert/explorer/testnet/tx/288150629f33de5d67a05b8a662e0546bbb9a018535678621e5788a0b5b718b4
https://lab.stellar.org/r/testnet/contract/CAQNZC2NW6PIUYNDE7WARU5XFFGOBKJXB5FI7ZEHDSKEYBMHD43BKGQB