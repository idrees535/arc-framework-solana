# ARC Framework Thesis
https://medium.com/@idrees535/adaptive-risk-coverage-arc-framework-a-dynamic-approach-of-next-order-value-aggregation-and-f2ec5cc58a77

# ARC Farmework Solana Implementation
This repository contains the implementation of the **Logarithmic Market Scoring Rule (LMSR)** programs, designed to provide dynamic pricing, inherent liquidity, and risk management for decentralized coverage pools.

## Prerequisites

Before you begin, ensure you have the following installed on your system:

- [Rust](https://www.rust-lang.org/tools/install): Install Rust and Cargo.
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools): Install and configure the Solana CLI.
- [Anchor Framework](https://project-serum.github.io/anchor/getting-started/installation.html): Install Anchor via Cargo.
- Node.js and Yarn: Required for Anchor dependencies.

```bash
# Install Anchor CLI
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --tag v0.25.0

# Verify installation
anchor --version
```

## Setting Up the Project

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/idrees535/arc-framework-solana
   cd arc-framework-solana
   ```

2. **Install Dependencies**:
   Navigate to the project directory and install dependencies.
   ```bash
   npm install
   ```

3. **Configure Solana CLI**:
   Set up your Solana CLI environment:
   ```bash
   solana config set --url localhost
   # or for devnet
   solana config set --url https://api.devnet.solana.com

   solana config get

   solana-keygen new --outfile ~/.config/solana/id.json --force
   ```
   If running locally, start a Solana validator:

   ```bash
   solana-test-validator
   solana airdrop 10

   ```

4. **Build the Program**:
   Compile the LMSR program:
   ```bash
   anchor build
   ```

5. **Deploy the Program**:
   Deploy the compiled LMSR program to the Solana blockchain.
   ```bash
   anchor deploy
   ```

6. **Set Program ID**:
   Update the `Anchor.toml` and `lib.rs` files with the deployed program ID:
   ```bash
   # Replace <PROGRAM_ID> with the actual deployed ID
   solana address -k target/deploy/prediction_market-keypair.json
   ```

   Update `Anchor.toml`:
   ```toml
   [programs.devnet]
   prediction_market = "<PROGRAM_ID>"
   ```

   Update `lib.rs`:
   ```rust
   declare_id!("<PROGRAM_ID>");
   ```

7. **Run Tests**:
   Execute the provided tests to ensure functionality:
   ```bash
   anchor test --skip-local-validator --skip-deploy
   ```

## Project Structure

- `programs/prediction_market/src`: Contains the LMSR program logic.
- `tests`: Integration tests for LMSR functionality.
- `Anchor.toml`: Anchor configuration file.
- `migrations/deploy.ts`: Script for program deployment.


## Contributing

Contributions are welcome! Please follow the [contribution guidelines](CONTRIBUTING.md) and open a pull request with your improvements.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
