# 🌐 Rust EVM Contracts Interactions

A Rust application to fetch and display Uniswap V2 factory pair addresses using the ethers library and the Uniswap V2 Factory smart contract. The main objective of this repository is to showcase how rust can be used to interact with EVM based smart contracts.

## 🚀 Features

- Fetches the total number of pools from the Uniswap V2 Factory contract.
- Retrieves pair addresses up to a specified limit.
- Displays results with colorful output.

## 🛠️ Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## 📦 Dependencies

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
ethers = "0.10.2"
eyre = "0.6"
tokio = { version = "1", features = ["full"] }
toml = "0.5"
colored = "2.0"
```

## 📂 Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── config.toml
├── contracts
│   └── IUniswapV2Factory.sol
├── src
│   └── main.rs
```

## ⚙️ Configuration

Create a `config.toml` file with your RPC endpoint:

```toml
[rpc]
web3_endpoints = [
"https://eth-mainnet.g.alchemy.com/v2/your_api_key",
]
```

## 🏃‍♂️ Running the Application

Clone the repository and navigate to the project directory:

```sh
git clone https://github.com/Aviksaikat/uniswap-v2-factory-pairs-fetcher.git
cd uniswap-v2-factory-pairs-fetcher
```

Run the application:

```sh
cargo run
```

## 👩‍💻 Usage

The application will fetch and display Uniswap V2 pair addresses with colorful output, helping you visualize the data easily.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgements

- [Ethers-rs](https://github.com/gakonst/ethers-rs)
- [Tokio](https://tokio.rs/)
