# blockchains-plus

A next-generation, Rust-based blockchain platform supporting user-deployable WASM smart contracts, custom tokens, staking, and a modern blockchain explorer.

## Features

- **Proof-of-Stake consensus** (inspired by Solana)
- **Native token** for payments, staking, and fun
- **WASM smart contract engine** (deploy custom tokens, DeFi, more)
- **User token creation**
- **Fast P2P networking**
- **Advanced blockchain explorer** (React-based)
- **Built with Rust for performance & safety**

## Structure

```
/node        — Blockchain core node (Rust)
/contracts   — Example smart contracts (Rust → WASM)
/explorer    — Web-based blockchain explorer (React)
/docs        — Documentation, guides
```

## Getting Started

1. `cargo build --workspace` to build core node & contracts.
2. See `/explorer/README.md` for web explorer setup.
3. Full docs coming soon!

---

## License

MIT (see `LICENSE`).