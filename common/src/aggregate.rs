use cil_messages::command::{Command};
use cil_messages::event::{DomainEvent};
use sp_core::{H160};
use sp_std::vec::{Vec};

pub trait AggregateState {
    fn on_evnt(&mut self, evnt: DomainEvent);
}

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

pub trait AggregateRepository {
    type Aggregate: Aggregate;

    fn get_aggregate(aggregate_id: H160) -> Self::Aggregate;
    fn save_aggregate(aggregate_id: H160, aggregate: Self::Aggregate);
}