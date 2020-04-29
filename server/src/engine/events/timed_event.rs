use core::cmp::Ordering;

use legion::prelude::*;

use super::event_data::EventData;
use crate::engine::game::Game;

pub struct TimedEvent {
    pub event_type: TimedEventType,
    pub execute_frame: u32,
    pub name: String,
    pub event_data: Option<EventData>,
    pub execute: fn(&mut Game) -> (),
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum TimedEventType {
    Once,
    Repeating(u32),
}

impl TimedEvent {
    pub fn new_repeated(&self, offset: u32) -> Self {
        Self {
            execute_frame: self.execute_frame + offset,
            name: self.name.clone(),
            event_data: self.event_data.clone(),
            ..*self
        }
    }
}

impl Ord for TimedEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        other.execute_frame.cmp(&self.execute_frame)
    }
}

impl PartialOrd for TimedEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TimedEvent {
    fn eq(&self, other: &Self) -> bool {
        self.execute_frame == other.execute_frame
    }
}

impl Eq for TimedEvent {}
