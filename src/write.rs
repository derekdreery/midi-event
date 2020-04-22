use crate::types::*;
use core::{i8, isize, ptr, slice};

pub trait Write<Output: AsMut<[u8]> + ?Sized> {
    /// If there is enough space, write to the Output and return the slice written to. Buffer
    /// contents can be anything if None is returned, and can be anything outside the slice.
    fn write(self, buffer: &mut Output) -> Option<&[u8]>;
}

impl Write<[u8]> for MidiEvent {
    #[inline]
    fn write(self, output: &mut [u8]) -> Option<&[u8]> {
        use crate::MidiEventType::*;
        debug_assert!(self.channel & 0x80 == 0);
        let channel = self.channel & 0x0F;
        match self.event {
            NoteOff(note, velocity) => {
                debug_assert!(velocity & 0x80 == 0);
                unsafe {
                    if output.len() < 3 {
                        return None;
                    }
                    *output.get_unchecked_mut(0) = 0x80 | channel;
                    *output.get_unchecked_mut(1) = note.into();
                    *output.get_unchecked_mut(2) = velocity & 0x7F;
                    Some(unsafe_slice(output, 0, 3))
                }
            }
            NoteOn(note, velocity) => {
                debug_assert!(velocity & 0x80 == 0);
                unsafe {
                    if output.len() < 3 {
                        return None;
                    }
                    *output.get_unchecked_mut(0) = 0x90 | channel;
                    *output.get_unchecked_mut(1) = note.into();
                    *output.get_unchecked_mut(2) = velocity & 0x7F;
                    Some(unsafe_slice(output, 0, 3))
                }
            }
            PolyphonicPressure(note, amount) => {
                debug_assert!(amount & 0x80 == 0);
                unsafe {
                    if output.len() < 3 {
                        return None;
                    }
                    *output.get_unchecked_mut(0) = 0xA0 | channel;
                    *output.get_unchecked_mut(1) = note.into();
                    *output.get_unchecked_mut(2) = amount & 0x7F;
                    Some(unsafe_slice(output, 0, 3))
                }
            }
            Controller(controller, value) => {
                debug_assert!(controller & 0x80 == 0);
                debug_assert!(value & 0x80 == 0);
                unsafe {
                    if output.len() < 3 {
                        return None;
                    }
                    *output.get_unchecked_mut(0) = 0xB0 | channel;
                    *output.get_unchecked_mut(1) = controller & 0x7F;
                    *output.get_unchecked_mut(2) = value & 0x7F;
                    Some(unsafe_slice(output, 0, 3))
                }
            }
            ProgramChange(program) => {
                debug_assert!(program & 0x80 == 0);
                unsafe {
                    if output.len() < 2 {
                        return None;
                    }
                    *output.get_unchecked_mut(0) = 0xC0 | channel;
                    *output.get_unchecked_mut(1) = program & 0x7F;
                    Some(unsafe_slice(output, 0, 2))
                }
            }
            ChannelPressure(pressure) => {
                debug_assert!(pressure & 0x80 == 0);
                unsafe {
                    if output.len() < 2 {
                        return None;
                    }
                    *output.get_unchecked_mut(0) = 0xD0 | channel;
                    *output.get_unchecked_mut(1) = pressure & 0x7F;
                    Some(unsafe_slice(output, 0, 2))
                }
            }
            PitchBend(lsb, msb) => {
                debug_assert!(lsb & 0x80 == 0);
                debug_assert!(msb & 0x80 == 0);
                unsafe {
                    if output.len() < 3 {
                        return None;
                    }
                    *output.get_unchecked_mut(0) = 0xE0 | channel;
                    *output.get_unchecked_mut(1) = lsb & 0x7F;
                    *output.get_unchecked_mut(2) = msb & 0x7F;
                    Some(unsafe_slice(output, 0, 3))
                }
            }
        }
    }
}

impl Write<[u8]> for Event<'_> {
    #[inline]
    fn write(self, output: &mut [u8]) -> Option<&[u8]> {
        match self {
            Event::Midi(evt) => evt.write(output),
            // TODO support messages longer than 127 (using variable length quantities).
            Event::SysEx(msg) => unsafe {
                if output.len() < msg.len() + 3 || msg.len() > i8::MAX as usize {
                    return None;
                }
                *output.get_unchecked_mut(0) = 0xF0;
                *output.get_unchecked_mut(1) = msg.len() as u8 + 1;
                ptr::copy_nonoverlapping(msg.as_ptr(), output.as_mut_ptr().offset(2), msg.len());
                *output.get_unchecked_mut(msg.len()) = 0xF7;
                Some(unsafe_slice(output, 0, msg.len() + 3))
            },
            // TODO support messages longer than 127 (using variable length quantities).
            Event::Escape(msg) => unsafe {
                if output.len() < msg.len() + 2 || msg.len() > i8::MAX as usize {
                    return None;
                }
                *output.get_unchecked_mut(0) = 0xF7;
                *output.get_unchecked_mut(1) = msg.len() as u8;
                ptr::copy_nonoverlapping(msg.as_ptr(), output.as_mut_ptr().offset(2), msg.len());
                Some(unsafe_slice(output, 0, msg.len() + 2))
            },
        }
    }
}

#[inline(always)]
unsafe fn unsafe_slice<T>(input: &[T], start: usize, end: usize) -> &[T] {
    debug_assert!(input.len() >= end);
    debug_assert!(start < end);
    debug_assert!(end < isize::MAX as usize);
    let pointer = input.as_ptr().offset(start as isize);
    slice::from_raw_parts(pointer, end - start)
}
