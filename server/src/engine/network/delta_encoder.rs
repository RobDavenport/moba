use crate::engine::messaging::messages::EntitySnapshot;
use std::collections::VecDeque;

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
        entities: &Vec<EntitySnapshot>,
    ) -> Option<(u32, Vec<EntitySnapshot>)> {
        if self.history.len() == SNAPSHOT_HISTORY_MAX_SIZE {
            println!("Snapshots are too old, clearing history");
            self.history.clear();
            self.ack_baseline = None;
            //return None;
        }

        let output = if let Some(baseline) = &self.ack_baseline {
            //calcualte the deltas, add them to out, and send
            //let mut out = Vec::new();
            //TODO fix this
            Some((baseline.frame, entities.clone()))
        } else {
            // We dont have a baseline, so just send a full snapshot
            None
        };

        self.history.push_back(SnapshotData {
            entity_data: entities.clone(),
            frame,
        });

        output
    }

    pub fn ack_baseline(&mut self, frame: u32) {
        //println!("got ack {}", frame);
        let drain_amount = if let Some(baseline) = &self.ack_baseline {
            if frame <= baseline.frame {
                //println!("packet out of order! got {} but already have {}", frame, baseline.frame);
                //we got an out-of-order packet
                return;
            }
            (frame - baseline.frame) - 1
        } else {
            //println!("ack {}, but refreshed", frame);
            0
        };

        //println!("will drain {}", drain_amount);
        self.history.drain(0..drain_amount as usize);
        self.ack_baseline = self.history.pop_front();

        // if let Some(baseline) = &self.ack_baseline {
        //     println!("baseline frame {}", baseline.frame);
        // }
    }
}
