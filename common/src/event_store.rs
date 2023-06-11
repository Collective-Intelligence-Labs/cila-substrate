use sp_core::{H160};
use sp_std::vec::{Vec};
use cil_messages::event::{DomainEvent};

pub trait EventStore {
    fn get(aggregate_id: H160) -> Vec<DomainEvent>;
    fn add(aggregate_id: H160, evnt: DomainEvent);
}