## Station emulator
*Work in progress*

![](https://github.com/romfrolov/station-emulator/workflows/build/badge.svg) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

Electric vehicle charging station emulator which supports OCPP 2.0.

[List of supported messages and use cases.](./SUPPORTED.md)

### Quick start

1. Create and configure `.env`.

```bash
# Copy the example.
cp .env.example .env
```

**Environment variables:**

- `CSMS_URL` - URL of Charging Station Management System (starting with *ws*).
- `STATION_ID` - ID that charging station will use to identify itself when communicating with CSMS.
- `SERIAL_NUMBER` - Serial number that charging station will use in `BootNotification` message.

2. Start the emulator.

```bash
cargo run
```

After that the station emulator should start and send a `BootNotification` message to CSMS.
