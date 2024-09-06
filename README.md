# rbca

Rustbucket Colour Adjustor. Game Boy/Game Boy Color emulator.

## Sources

Made with help from the [Pan Docs specifications](http://bgb.bircd.org/pandocs.htm#cgbregisters).

## Blargg Test Status

### CPU Instruction Tests

- 01: Passed
- 02: Hangs indefinitely (possibly same problem as 07)
- 03: Passed
- 04: Passed
- 05: Passed
- 06: Passed
- 07: Hangs indefinitely (@ 0xC738 - LDH A,(n) - Reads 0x00 from 0xFF44 instead of 0x90)
- 08: Passed
- 09: Passed
- 10: Passed
- 11: Passed
