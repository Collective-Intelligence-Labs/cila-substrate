# NFTsAggregate Pallet Documentation

The `NFTsAggregate` pallet is designed to manage and handle Non-Fungible Tokens (NFTs) within the Substrate framework. It provides functionality for minting and transferring NFTs, while also emitting relevant domain events. This documentation provides an overview of the pallet's structure, its functions, and example usages.

## Table of contents
- [Overview](#overview)
  - [Dependencies](#dependencies)
- [Pallet Structure](#pallet-structure)
- [Example Usages](#example-usages)
  - [Minting an NFT](#minting-an-nft)
  - [Transferring an NFT](#transferring-an-nft)
- [Conclusion](#conclusion)

## Overview

The `NFTsAggregate` pallet is implemented as an Aggregate, which is a design pattern that combines state and event handling for managing NFTs. The pallet maintains the current state of NFTs, tracks changes, and emits domain events related to NFT minting and transfers.

### Dependencies

This pallet relies on several external crates and modules:
- `sp_std::vec::Vec`: A collection for dynamic array storage.
- `cil_messages::command` and `cil_messages::event`: Modules that define command and event structures for interacting with NFTs.
- `cil_messages::utils`: Utilities for serializing and deserializing payload data.
- `cil_common::aggregate`: A module providing the `Aggregate` trait, which this pallet implements.
- `NftsAggregateState`: A state structure specific to this pallet.

## Pallet Structure

The `NFTsAggregate` pallet is structured as follows:

- `NftsAggregate` struct: Holds the state, changes, and event count of the pallet.
  - `state`: The current state of the NFTs.
  - `changes`: A vector storing domain events.
  - `evnts_count`: The count of emitted events.

- Implementation of the `Aggregate` trait for `NftsAggregate`:
  - `get_state()`: Returns a reference to the pallet's state.
  - `get_state_mut()`: Returns a mutable reference to the pallet's state.
  - `get_evnts_count()`: Returns the count of emitted events.
  - `inc_evnts_count()`: Increments the event count.
  - `handle_command(cmd: Command)`: Handles incoming commands, specifically minting and transferring NFTs.
  - `pull_changes()`: Returns a clone of the vector containing domain events.
  - `put_change(evnt: DomainEvent)`: Appends a domain event to the vector of changes.

- Additional methods specific to `NftsAggregate`:
  - `mint(payload: MintNFTPayload) -> Result<(), &str>`: Handles NFT minting, emits the `NFT_MINTED` event.
  - `transfer(payload: TransferNFTPayload) -> Result<(), &str>`: Handles NFT transfer, emits the `NFT_TRANSFERED` event.

## Example Usages

### Minting an NFT

To mint a new NFT, a `MintNFTPayload` is provided. The payload should include the NFT's hash and owner. Here's an example of how to mint an NFT:

```rust
let payload = MintNFTPayload {
    hash: "nft_hash".to_string(),
    owner: "owner_account".to_string(),
};

let mut nfts_aggregate = NftsAggregate::default();
nfts_aggregate.mint(payload).expect("Failed to mint NFT");
```

### Transferring an NFT
To transfer an NFT, a `TransferNFTPayload` is provided. The payload should include the NFT's hash and the receiver's address. Here's an example of how to transfer an NFT:

```rust
let payload = TransferNFTPayload {
    hash: "nft_hash".to_string(),
    receiver: "receiver_account".to_string(),
};

let mut nfts_aggregate = NftsAggregate::default();
nfts_aggregate.transfer(payload).expect("Failed to transfer NFT");
```

## Conclusion
The NFTsAggregate pallet provides an efficient and flexible way to manage NFTs within the Substrate framework. It handles both minting and transferring NFTs while emitting relevant domain events. By following the provided examples, developers can seamlessly integrate NFT management into their Substrate-based projects.