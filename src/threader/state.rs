use std::{thread::{Builder, JoinHandle}, vec::Vec, sync::{Mutex, Arc}, num::NonZero};

pub struct ThreadState<'a> {
    pub _id: u32,
    pub current_idx: u32,
    pub controller: &'a ControllerState,
}

impl<'a> ThreadState<'a> {
    pub fn new(id: u32, controller: &'a ControllerState) -> Self {
        let new = ThreadState {
            _id: id,   
            current_idx: 0,
            controller: controller    
        };
        return new;
    }
}

pub struct ControllerState {
    pub indir: String,
    pub outdir: String,
    pub script: String,
    
    pub frame_count: u32,
    pub frame_index: Mutex<u32>,

    pub threads: Mutex< Vec< JoinHandle<()> > >,
}

impl ControllerState {
    pub fn new(indir: String, outdir: String, script: String, frame_count: u32) -> Arc<Self> {
        let new = Arc::new(ControllerState {
            indir: indir,
            outdir: outdir,
            script: script,

            frame_index: Mutex::new(0),
            frame_count: frame_count,

            threads: Mutex::new(Vec::new()),
        }) ;
        return new;
    }

    pub fn start(self: &Arc<Self>, thread_count: NonZero<usize>) -> () {
        let mut threads = self.threads.lock().unwrap();
        
        if !threads.is_empty() {
            return;
        }

        for idx in 0..thread_count.into() {
            let controller = Arc::clone(self);
            
            let handle = Builder::new()
                .name(format!("Worker-{}", idx))
                .spawn(move || {
                    let mut thread_state = ThreadState::new(idx as u32, &controller);
                    super::workload::thread_task(&mut thread_state);
                })
                .expect("Failed to spawn thread");
            threads.push(handle);
        }
    }

    pub fn join(self: &Arc<Self>) -> () {
        let mut threads = self.threads.lock().unwrap();
        while let Some(handle) = threads.pop() {
            handle.join().unwrap();
        }
    }

    pub fn get_next_frame(&self) -> Option<u32> {
        let mut frame_index = self.frame_index.lock().unwrap();
        if *frame_index >= self.frame_count {
            None
        } else {
            let idx = *frame_index;
            *frame_index += 1;
            Some(idx)
        }
    }
}