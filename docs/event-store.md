# Event Store Pallet Documentation

- [Overview](#overview)
- [Getting Started](#getting-started)
  - [Dependencies](#dependencies)
  - [Installation](#installation)
- [Usage](#usage)
  - [Event Structure](#event-structure)
  - [Event Store Interface](#event-store-interface)
- [Events](#events)
  - [Example Event](#example-event)
- [Examples](#examples)
  - [Example Usage](#example-usage)
- [References](#references)

## Overview
The `event-store` pallet is responsible for storing event streams that occur to aggregates, such as Non-Fungible Tokens (NFTs), in a Substrate-based blockchain. This documentation provides a comprehensive guide to understand the functionality, usage, and customization options of the `event-store` pallet.

### Getting Started
To utilize the `event-store` pallet, ensure that the following dependencies are met:
- Rust (nightly version)
- Substrate framework

#### Dependencies
- Rust (nightly version)
- Substrate framework

#### Installation
Follow these steps to include the `event-store` pallet in your Substrate-based project:
1. Add the following lines to your `Cargo.toml` file:
   ```
   [dependencies]
   event-store-pallet = { git = "https://github.com/Collective-Intelligence-Labs/cila-substrate", branch = "main", package = "event-store-pallet" }
   ```
2. In your runtime's `lib.rs`, include the following:
   ```
   use event_store_pallet::{Event, EventStoreInterface};
   ```
3. Implement the necessary logic to interact with the event store.

## Usage

### Event Structure
Events represent individual occurrences or changes that happen to aggregates. They capture relevant information about the change and are stored in the event store.
```
pub struct Event {
    // Define event fields here
}
```

### Event Store Interface
The `EventStoreInterface` trait provides the necessary functions to interact with the event store.

#### Example Implementation
Here's an example implementation of the `EventStoreInterface` trait:
```
pub struct MyEventStore;

impl EventStoreInterface<Runtime> for MyEventStore {
    fn store_event(event: &Event) {
        // Custom logic for storing the event
    }
}
```

## Events
The `event-store` pallet emits events to notify subscribers about important occurrences related to event storage. Events are defined using the `#[pallet::event]` attribute macro.

### Example Event
Here's an example event definition associated with event storage:
```
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    EventStored(Event),
    // Define other events here
}
```

## Examples

### Example Usage
Here's an example usage scenario showcasing how to integrate the `event-store` pallet into your runtime:
```
// Import necessary dependencies

// Define your runtime and configuration types

// Include the necessary modules from the `event-store` pallet

// Implement event storage logic

// Store events
```

## References
For more detailed information and examples, refer to the provided references and resources.
