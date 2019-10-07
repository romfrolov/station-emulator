## Station emulator
*Work in progress*

![](https://github.com/romfrolov/station-emulator/workflows/Rust/badge.svg)

Electric vehicle charging station emulator which supports OCPP 2.0.

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
  - [ ] - ChangeAvailability
  - [ ] - SetVariables
- [ ] reservations
  - [ ] - ReserveNow
  - [ ] - CancelReservation

### Known issues

1. Message synchronicity.
