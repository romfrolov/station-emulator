## Station emulator
*Work in progress*

![](https://github.com/romfrolov/station-emulator/workflows/build/badge.svg)

Electric vehicle charging station emulator which supports OCPP 2.0.

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

### List of supported messages

- [x] boot
  - [x] - BootNotification
  - [x] - Heartbeat
  - [x] - StatusNotification
- [x] transaction
  - [x] - TransactionEvent
  - [x] - RequestStartTransaction (only remote authorization)
  - [x] - RequestStopTransaction
- [ ] settings
  - [x] - SetVariables
  - [x] - GetVariables
  - [ ] - ChangeAvailability
- [ ] reservations
  - [ ] - ReserveNow
  - [ ] - CancelReservation

### Wishlist

1. Configuration of station's EVSE and connectors.
