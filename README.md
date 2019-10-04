## Station emulator
*Work in progress*

Electric vehicle charging station emulator which supports OCPP 2.0.

### List of supported messages

**boot**
- [x] - BootNotification
- [x] - Heartbeat
- [x] - StatusNotification
**transaction**
- [ ] - TransactionEvent
- [ ] - RequestStartTransaction (only remote authorization)
- [ ] - RequestStopTransaction
**settings**
- [ ] - ChangeAvailability
- [ ] - SetVariables
**reservations**
- [ ] - ReserveNow
- [ ] - CancelReservation
