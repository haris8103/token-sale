# 🪙 Solana Token Vesting Program

This Solana smart contract implements a simple token vesting and sale mechanism. It allows a token creator to mint a fixed supply of SPL tokens to a program-derived account (PDA), set the initial price on the first purchase, and allow others to buy tokens at that price.

---

## 📦 Features

- ✅ Create an SPL token mint and vest full supply to a PDA
- 🧠 PDA-based authority and state management
- 🪙 First buyer sets the price via `bps` (basis points)
- 💸 Subsequent buyers purchase tokens at fixed price
- 🔐 Minting is disabled permanently after initialization
- 🪪 ATA (Associated Token Account) creation included

---

## 📚 Instructions

### `Create` (Instruction 0)

Initializes:
- A new SPL Token mint
- The PDA’s BuyConfig state
- Mints total supply to the PDA's ATA
- Disables mint authority forever



#### Args 
{
  instruction: 0,
  token_decimals: u8,
  token_supply: u64,
}

### FirstBuy (Instruction 1)

Sets token price based on a buyer’s payment and bps (percent of supply). Mints a fraction of tokens to the buyer and sets the price permanently.

#### Args
{
  instruction: 1,
  amount: u64, // SOL paid
  bps: u8      // Portion of total supply (e.g. 10 = 10%)
}

### Buy (Instruction 2)

Allows subsequent buyers to purchase tokens at the fixed price set in FirstBuy.


####
{
  instruction: 2,
  amount: u64, // SOL paid
}
