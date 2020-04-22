# Goals

 - Correct for correct input.
 - Only events - not files.
 - Fast and easy for compiler to inline/optimize. (This includes ignoring the top bit rather than checking
   it, for example).
 - Ideally as fast as working with raw bytes, but more ergonomic.
 - Memory safe, even on bad input (this is the time we are willing to sacrifice performance).
 - `#[no_std]`

# Stretch

 - Deal with malformed messages
 - Deal with common sysex messages, but not in a way that harms performance of the standard case.

# Non-goals

 - Parsing smf files
 - Easy to understand code
 - `#![unsafe(never)]`
