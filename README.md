# Subxt Client (for benchmarking)
This is a small Rust client built using Subxt to interact with a Substrate-based chain.  
The main goal of this project is to measure different aspects of transaction performance.

## What it does
- Create accounts using a seed
- Submits balance transfer transactions
- Measures:
  - time to inclusion in a block
  - time to finalisation
  - signing time
  - extrinsic size and weight
- Writes results to CSV for later analysis

## Setup
Make sure you have a Substrate node running locally: <br>
_ws://127.0.0.1:9944_

Fr this testing, I used polkadot-sdk-solochain-template to run a local node. 

Then generate metadata (You should have subxt-cli installed): <br>
_subxt metadata --url ws://127.0.0.1:9944 > solochain_metadata.scale_

Create the chain.rs and chain_readable.rs inside the /src folder. Three files: solochain_metadata.scale, chain.rs and chain_readable.rs are running node-specific and should be replaced by newly created files for your specific running node. 

_subxt codegen --url ws://127.0.0.1:9944 > src/chain.rs_ <br>
_cp src/chain.rs src/chain_readable.rs_ <br>
_rustfmt src/chain_readable.rs_


## Build and Run
_cargo build --release_

Once the build is successful, run the client <br>
_cargo run_


## Disclaimer Note

- The client is only for experimentation and not for production use. Not all aspects of the client are tested, and any use of this code for production or experiments will be at the user's own risk. 

## Structure

- `main.rs` → entry point
- `helpers.rs` → utility functions (nonce, RPC helpers, etc.)
- `measuring_functions.rs` → benchmarking logic
- 
