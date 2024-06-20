# Triangular Arbitrage Algorithm in Rust

This project implements a triangular arbitrage algorithm in Rust, leveraging live cryptocurrency data stored in Redis streams. Triangular arbitrage involves exploiting price discrepancies between three different cryptocurrencies to achieve profit through trading.

## Features

- **Real-time Data Processing**: Utilizes Redis streams to fetch live cryptocurrency price data.
- **Triangular Arbitrage Strategy**: Implements an algorithm to detect and calculate potential profit from arbitrage opportunities.
- **Flexibility**: Easily extendable to include additional cryptocurrencies or adapt to different trading strategies.

## Requirements

- Rust programming language (install from [rust-lang.org](https://www.rust-lang.org))
- Redis server (install from [redis.io](https://redis.io/download))

## Installation

1. Clone the repository:

   ```bash
   git clone <repository_url>
   cd <repository_directory>
