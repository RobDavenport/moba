use crate::engine::messaging::messages::EntitySnapshot;
use std::collections::VecDeque;

use itertools::{diff_with, Diff};

const SNAPSHOT_HISTORY_MAX_SIZE: usize = 64;

pub struct SnapshotHistory {
    history: VecDeque<SnapshotData>,
    ack_baseline: Option<SnapshotData>,
}

struct SnapshotData {
    pub entity_data: Vec<EntitySnapshot>,
    pub frame: u32,
}

impl SnapshotHistory {
    pub fn new() -> Self {
        Self {
            history: VecDeque::with_capacity(SNAPSHOT_HISTORY_MAX_SIZE),
            ack_baseline: None,
        }
    }

    pub fn encode_delta(
        &mut self,
        frame: u32,
        entities: &[EntitySnapshot],
    ) -> Option<(u32, Vec<EntitySnapshot>)> {
        if self.history.len() == SNAPSHOT_HISTORY_MAX_SIZE {
            drop(self.history.pop_front());
            self.ack_baseline = None;
        }

        let output = if let Some(baseline) = &self.ack_baseline {
            let mut out = Vec::<EntitySnapshot>::new();
            let mut base_iter = baseline.entity_data.iter();
            let mut next_iter = entities.iter();

            while let Some(diff) = diff_with(base_iter, next_iter, |base, next| base == next) {
                match diff {
                    Diff::FirstMismatch(_len, i, j) => {
                        //Mismatch found at index
                        let (i_val, i_iter) = i.into_parts();
                        let (j_val, j_iter) = j.into_parts();
                        out.push(get_entity_delta(i_val.unwrap(), j_val.unwrap()));
                        base_iter = i_iter;
                        next_iter = j_iter;
                    }
                    Diff::Shorter(_len, _i) => {
                        // TODO: Do we need this?
                        // Next contains less elementas than baseline
                        break;
                    }
                    Diff::Longer(_len, j) => {
                        // Next contains more elements than baseline
                        for snapshot in j {
                            out.push(snapshot.clone());
                        }
                        break;
                    }
                }
            }
            Some((baseline.frame, out))
        } else {
            // We dont have a baseline, so just send a full snapshot
            None
        };

        self.history.push_back(SnapshotData {
            entity_data: entities.to_vec(),
            frame,
        });

        output
    }

    pub fn ack_baseline(&mut self, frame: u32) {
        if let Some(baseline) = &self.ack_baseline {
            if frame <= baseline.frame {
                // Early out for out-of-order packet
                return;
            }
            let drain_amount = (frame - baseline.frame) - 1;
            self.history.drain(0..drain_amount as usize);
        } else {
            while let Some(popped) = self.history.front() {
                if popped.frame == frame {
                    break;
                } else {
                    self.history.pop_front();
                }
            }
        }

        self.ack_baseline = self.history.pop_front();
    }
}

fn get_entity_delta(baseline: &EntitySnapshot, next: &EntitySnapshot) -> EntitySnapshot {
    EntitySnapshot {
        replication_id: next.replication_id,
        rotation: get_delta_field(&baseline.rotation, &next.rotation),
        x: get_delta_field(&baseline.x, &next.x),
        y: get_delta_field(&baseline.y, &next.y),
        energy: get_delta_field(&baseline.energy, &next.energy),
        health: get_delta_field(&baseline.health, &next.health),
        entity_type: get_delta_field(&baseline.entity_type, &next.entity_type),
    }
}

fn get_delta_field<T: PartialEq + Clone>(base: &Option<T>, next: &Option<T>) -> Option<T> {
    match (base, next) {
        (Some(b), Some(n)) => {
            if b != n {
                Some(n.clone())
            } else {
                None
            }
        }
        (None, Some(n)) => Some(n.clone()),
        _ => None,
    }
}
