# Dispatcher Pallet Documentation

The `dispatcher` pallet handles commands written in Protobuf format and dispatches them to the corresponding registered command handlers. This documentation provides a comprehensive guide to understand the functionality, usage, and customization options of the `dispatcher` pallet.

## Table of Contents
1. [Overview](#overview)
2. [Getting Started](#getting-started)
   - [Dependencies](#dependencies)
   - [Installation](#installation)
3. [Usage](#usage)
   - [Command Structure](#command-structure)
   - [Command Handlers](#command-handlers)
     - [Example Command Handler](#example-command-handler)
   - [Dispatching Commands](#dispatching-commands)
4. [Events](#events)
   - [Example Event](#example-event)
5. [Customization](#customization)
   - [Adding New Command Handlers](#adding-new-command-handlers)
6. [Examples](#examples)
   - [Example Usage](#example-usage)
7. [References](#references)

## Overview

The `dispatcher` pallet allows seamless integration of Protobuf commands into a Substrate-based blockchain. It provides the necessary infrastructure to parse incoming commands, validate them, and route them to the corresponding registered command handlers.

**Key Features:**
- Parsing and validation of commands written in Protobuf format
- Dynamic dispatching of commands to registered command handlers
- Event emission to notify subscribers about command execution

## Getting Started

### Dependencies

To utilize the `dispatcher` pallet, ensure that the following dependencies are met:
- Rust (nightly version)
- Substrate framework

### Installation

Follow these steps to include the `dispatcher` pallet in your Substrate-based project:

1. Add the following lines to your `Cargo.toml` file:

```
[dependencies]
dispatcher-pallet = { git = "https://github.com/Collective-Intelligence-Labs/cila-substrate", branch = "main", package = "dispatcher-pallet" }
```

2. In your runtime's `lib.rs`, include the following:

```
use dispatcher_pallet::{Command, CommandHandler, CommandDispatcher, Event};
```

3. Implement the necessary command handlers for your specific use case.

## Usage

### Command Structure

Commands are written in Protobuf format and need to be parsed into a `Command` struct before dispatching.

```
pub struct Command {
    // Define command fields here
}
```

### Command Handlers

Command handlers are responsible for executing logic based on the command type. Implement the `CommandHandler` trait for each command type to define the desired behavior.

#### Example Command Handler

Here's an example implementation of a command handler for a specific command type:

```
pub struct MyCommandHandler;

impl CommandHandler<Runtime> for MyCommandHandler {
    fn handle_command(command: &Command) -> Result<(), DispatchError> {
        // Custom logic for handling the command
        Ok(())
    }
}
```

### Dispatching Commands

To dispatch commands to the corresponding command handlers, follow these steps:

1. Parse the incoming command using the `parse_command` function.

```
let parsed_command = parse_command(raw_command);
```

2. Create an instance of `CommandDispatcher` and register the desired command handlers.

```
let mut dispatcher = CommandDispatcher::<Runtime>::new();
dispatcher.register_command_handler::<MyCommandHandler>();
```

3. Dispatch the parsed command.

```
dispatcher.dispatch(&parsed_command)?;
```

## Events

The `dispatcher` pallet emits events to notify subscribers about important occurrences during command execution. Events are defined using the `#[pallet::event]` attribute macro.

### Example Event

Here's an example event definition associated with command execution:

```
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    CommandExecuted(Command),
    // Define other events here
}
```

## Customization

### Adding New Command Handlers

To add new command handlers to the `dispatcher` pallet, follow these steps:

1. Implement the desired command handler by implementing the `CommandHandler` trait.

2. Register the command handler in the `CommandDispatcher` using the appropriate method or function.

Now, the `dispatcher` pallet can dispatch commands to the newly added command handler.

## Examples

### Example Usage

Here's an example usage scenario showcasing how to integrate the `dispatcher` pallet into your runtime:

```
// Import necessary dependencies

// Define your runtime and configuration types

// Include the necessary modules from the `dispatcher` pallet

// Implement a custom command handler

// Dispatching commands
```

## References

For more detailed information and examples, refer to the provided references and resources.
