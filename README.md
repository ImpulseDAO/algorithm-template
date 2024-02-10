# Arena Algorithm Template
This repository is supposed to be a template for a new arena algorithm.

# How to use
1. Create a new repository on top of the template.
2. Describe your character's behaviour in `src/lib.rs`.
3. Build the project via `cargo build --release`.
4. After successful build your algorithm can be found by the following path `target/wasm32-unknown-unknown/<algorithm-name>.opt.wasm`

# How to test
I guess the best way is to deploy the new algorithm to vara mainnet or testnet (arena contracts should be presented on both) and test it in battle!
