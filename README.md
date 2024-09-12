# rbca

Rustbucket Colour Adjustor. Game Boy/Game Boy Color emulator.

## Sources

Made with help from the [Pan Docs specifications](http://bgb.bircd.org/pandocs.htm#cgbregisters).

## Blargg Test Status

### CPU Instruction Tests

- 01: Passed
- 02: Passed
- 03: Passed
- 04: Passed
- 05: Passed
- 06: Passed
- 07: Passed
- 08: Passed
- 09: Passed
- 10: Passed
- 11: Passed

### Instruction Timing Tests

- Passed

### Memory Timing Tests

- 01: Failed
- 02: Failed
- 03: Failed

### Memory-2 Timing Tests

- 01: Failed
- 02: Failed
- 03: Failed

### Interrupt Timing Test

- Failed

### DMG Sound Tests

- 01: Untested
- 02: Untested
- 03: Untested
- 04: Untested
- 05: Untested
- 06: Untested
- 07: Untested
- 08: Untested
- 09: Untested
- 10: Untested
- 11: Untested
- 12: Untested

### OAM Bug Tests

- 1: Turning LCD on starts too late in scanline (Failed #2)
- 2: LD DE, $FE00 : INC DE (Failed #2)
- 3: Passed
- 4: INC DE at first corruption (Failed #3)
- 5: Should corrupt at beginning of first scanline (Failed #2)
- 6: Passed
- 7: 00000000 Failed
- 8: 00000000 INC/DEC rp pattern is wrong (Failed #2)

### Halt Bug Test

- Failed
