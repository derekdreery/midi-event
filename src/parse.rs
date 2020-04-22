use crate::types::*;

use core::{mem, slice};

/// Go from raw input into a midi event.
pub trait Parse<Input>: Sized {
    fn parse(input: Input) -> Option<Self>;
}

impl<'src> Parse<&'src [u8]> for Event<'src> {
    #[inline]
    fn parse(input: &[u8]) -> Option<Event> {
        let first = input.get(0)?;
        match first & 0xF0 {
            0x80 => {
                let velocity = *input.get(2)?;
                let note = *input.get(1)?;
                Some(Event::Midi(MidiEvent {
                    channel: first & 0x0F,
                    event: MidiEventType::NoteOff(note.into(), velocity & 0x7F),
                }))
            }
            0x90 => {
                let velocity = *input.get(2)?;
                let note = *input.get(1)?;
                Some(Event::Midi(MidiEvent {
                    channel: first & 0x0F,
                    event: MidiEventType::NoteOn(note.into(), velocity & 0x7F),
                }))
            }
            0xA0 => {
                let pressure = *input.get(2)?;
                let note = *input.get(1)?;
                Some(Event::Midi(MidiEvent {
                    channel: first & 0x0F,
                    event: MidiEventType::PolyphonicPressure(note.into(), pressure & 0x7F),
                }))
            }
            0xB0 => {
                let value = *input.get(2)?;
                let controller = *input.get(1)?;
                Some(Event::Midi(MidiEvent {
                    channel: first & 0x0F,
                    event: MidiEventType::Controller(controller & 0x7F, value & 0x7F),
                }))
            }
            0xC0 => {
                let program = *input.get(1)?;
                Some(Event::Midi(MidiEvent {
                    channel: first & 0x0F,
                    event: MidiEventType::ProgramChange(program & 0x7F),
                }))
            }
            0xD0 => {
                let pressure = *input.get(1)?;
                Some(Event::Midi(MidiEvent {
                    channel: first & 0x0F,
                    event: MidiEventType::ChannelPressure(pressure & 0x7F),
                }))
            }
            0xE0 => {
                let msb = *input.get(2)?;
                let lsb = *input.get(1)?;
                Some(Event::Midi(MidiEvent {
                    channel: first & 0x0F,
                    event: MidiEventType::PitchBend(lsb & 0x7F, msb & 0x7F),
                }))
            }
            0xF0 => match first & 0x0F {
                0x00 => {
                    let mut end_idx = 0;
                    loop {
                        match input.get(end_idx) {
                            Some(0xF7) | None => break,
                            _ => (),
                        }
                        end_idx += 1;
                    }
                    unsafe {
                        let pointer = input.as_ptr().offset(1);
                        Some(Event::SysEx(slice::from_raw_parts(pointer, end_idx - 1)))
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }
}

impl Parse<&'_ [u8]> for MidiEvent {
    #[inline]
    fn parse(input: &[u8]) -> Option<MidiEvent> {
        match Event::parse(input) {
            Some(Event::Midi(evt)) => Some(evt),
            _ => None,
        }
    }
}

impl Parse<u8> for Note {
    #[inline]
    fn parse(input: u8) -> Option<Self> {
        if input & 0x80 == 0x80 {
            None
        } else {
            Some(unsafe { mem::transmute(input) })
        }
    }
}
