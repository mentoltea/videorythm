use super::state::{ThreadState};
use crate::image::processor::{open_edit_save};

pub fn thread_task(state: &mut ThreadState) -> () {
    let mut run = true;

    while run {
        match state.controller.get_next_frame() {
            Some(x) => {
                state.current_idx = x;
                open_edit_save(
                    state.controller.indir.clone(), 
                    state.controller.outdir.clone(), 
                    state.controller.script.clone(),
                    state.current_idx,
                    state.controller.frame_count
                );
            }
            None => {run = false;}
        }
    }
    return;
}