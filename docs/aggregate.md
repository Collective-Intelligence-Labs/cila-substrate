# CIL Aggregate Module Documentation

This documentation provides an overview of the Aggregate module in the CIL framework. 

## Dependencies

```rust
use cil_messages::command::{Command};
use cil_messages::event::{DomainEvent};
use sp_core::{H160};
use sp_std::vec::{Vec};
```

## Traits

### `AggregateState`

This trait represents the state of an aggregate and its ability to process a domain event.

```rust
pub trait AggregateState {
    fn on_evnt(&mut self, evnt: DomainEvent);
}
```

### `Aggregate`

The primary trait for Aggregates, which includes functions for handling commands, applying and retrieving events, and managing the internal state.

```rust
pub trait Aggregate {
    type AggregateState: AggregateState;

    fn get_state(&self) -> &Self::AggregateState;
    fn get_state_mut(&mut self) -> &mut Self::AggregateState;
    fn handle_command(&mut self, cmd: Command);
    fn pull_changes(&self) -> Vec<DomainEvent>;
    fn put_change(&mut self, evnt: DomainEvent);
    fn get_evnts_count(&self) -> u64;
    fn inc_evnts_count(&mut self);
    fn apply_event(&mut self, evnt: DomainEvent) {
        self.get_state_mut().on_evnt(evnt.clone());
        self.inc_evnts_count();
        self.put_change(evnt);
    }
    fn replay_events(&mut self, evnts: Vec<DomainEvent>) {
        for evnt in evnts.into_iter() {
            self.get_state_mut().on_evnt(evnt);
            self.inc_evnts_count();
        }
    }
}
```

### `AggregateRepository`

A trait that defines the basic operations to retrieve and store aggregates.

```rust
pub trait AggregateRepository {
    type Aggregate: Aggregate;

    fn get_aggregate(aggregate_id: H160) -> Self