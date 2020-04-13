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
        entities: &Vec<EntitySnapshot>,
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
                        let (j_val, j_iter) = j.into_parts();
                        out.push(j_val.unwrap().clone());
                        base_iter = i.into_parts().1;
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
        //     println!("delta = +{} || {} {}", frame - baseline.frame, frame, baseline.frame);
        //     //println!("baseline frame {}", baseline.frame);
        // }
    }
}
