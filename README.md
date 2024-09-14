# rbca

Rustbucket Colour Adjustor. Game Boy/Game Boy Color emulator made from scratch.

## Current Status

DMG mode only for now.

Boots fine, seems to run Tetris, Bomberman GB, Tennis, Link's Awakening fine.

Some games don't do anything without a boot ROM.

Dr. Mario freezes after pressing START on game select screen.

Appears to run somewhat slow sometimes, e.g. ~48 FPS in Kirby's Dream Land. Need to test whether this is due to the (likely inefficient) SDL2 Desktop frontend rendering method, or for another reason.

Screen can have horizontal tears sometimes.

A few layering issues.

No reset button, audio, serial data transfer, or save files... yet!

Supported cartridge types: ROM, MBC1

Yet-to-be-supported cartridge types: MBC2, MMM01, MBC3, MBC5, MBC6, MBC7, POCKET CAMERA, BANDAI TAMA5, HuC3, HuC1

## Sources

Made with help from the [Pan Docs specifications](http://bgb.bircd.org/pandocs.htm#cgbregisters).

## Tests Status

### Blargg's CPU Instruction Tests

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

### Blargg's Instruction Timing Tests

- Passed

### Blargg's Memory Timing Tests

- 01: Failed
- 02: Failed
- 03: Failed

### Blargg's Memory-2 Timing Tests

- 01: Failed
- 02: Failed
- 03: Failed

### Blargg's Interrupt Timing Test

- Failed

### Blargg's DMG Sound Tests

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

### Blargg's OAM Bug Tests

- 1: Turning LCD on starts too late in scanline (Failed #2)
- 2: LD DE, $FE00 : INC DE (Failed #2)
- 3: Passed
- 4: INC DE at first corruption (Failed #3)
- 5: Should corrupt at beginning of first scanline (Failed #2)
- 6: Passed
- 7: 00000000 Failed
- 8: 00000000 INC/DEC rp pattern is wrong (Failed #2)

### Blargg's Halt Bug Test

- Failed

### dmg-acid2

- Fail hair visible: background enable (bit 0)
- Fail tongue visible: object enable (bit 1)
- Fail half of mouth missing: object size (bit 2)
- Fail footer missing: background tile map (bit 3)
- Fail right chin missing: window tile map (bit 6)
- Fail left eye mole visible: background tile data is read from 0x8000-0x8FFF instead of 0x8800-97FF.
