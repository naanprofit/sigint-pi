# TODO: Soundboard Module & Fast Food / Commercial RF Monitoring

See full specification in project documentation.
This file tracks implementation status.

## Implementation Status

### Soundboard Module
- [x] Backend: clip storage, upload, list, delete
- [x] Backend: local playback via aplay
- [x] Backend: RF transmit via HackRF+csdr (safety interlocks)
- [x] Backend: TX frequency blocklist (cellular, aviation)
- [x] Frontend: Soundboard tab with clip grid
- [x] Frontend: upload, play, transmit controls
- [ ] Frontend: assign clips to alert events
- [ ] SIEM integration for TX audit log

### Fast Food / Commercial RF Module
- [x] Frequency database (intercoms, pagers, FRS/MURS, digital)
- [x] Backend: passive scan across all bands
- [x] Backend: signal classification
- [x] Frontend: Commercial RF tab
- [ ] POCSAG pager decode via multimon-ng
- [ ] DMR metadata decode via dsd-fme
- [ ] FRS/MURS TX support
- [ ] DECT 1.9 GHz detect-only support

### Dependencies
- [ ] Install csdr on Pi and Deck
- [ ] Install multimon-ng on Pi and Deck
- [ ] Install sox on Pi and Deck
