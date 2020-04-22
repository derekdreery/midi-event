#![no_main]
use libfuzzer_sys::fuzz_target;
use midi_event::*;

fuzz_target!(|data: &[u8]| {
    let _ = Event::parse(data);
});

