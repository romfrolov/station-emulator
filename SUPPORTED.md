# Supported OCPP 2.0 messages and use cases

## Supported messages
- BootNotification
- Heartbeat
- StatusNotification
- TransactionEvent
- RequestStartTransaction (only remote authorization)
- RequestStopTransaction
- SetVariables
- GetVariables

## Supported use cases
| Subject                           | Use Case                                                                    | Supported | Comments                                      |
| :--------------------------------:| :--------------------------------------------------------------------------:| :--------:| :--------------------------------------------:|
| Security                          | A01 - Update Charging Station Password for HTTP Basic Authentication        |           |                                               |
| Security                          | A02 - Update Charging Station Certificate by request of CSMS                |           |                                               |
| Security                          | A03 - Update Charging Station Certificate initiated by the Charging Station |           |                                               |
| Security                          | A04 - Security Event Notification                                           |           |                                               |
| Provisioning                      | B01 - Cold Boot Charging Station                                            | Yes       |                                               |
| Provisioning                      | B02 - Cold Boot Charging Station - Pending                                  |           |                                               |
| Provisioning                      | B03 - Cold Boot Charging Station - Rejected                                 |           |                                               |
| Provisioning                      | B04 - Offline Behavior Idle Charging Station                                |           |                                               |
| Provisioning                      | B05 - Set Variables                                                         |           |                                               |
| Provisioning                      | B06 - Get Variables                                                         |           |                                               |
| Provisioning                      | B07 - Get Base Report                                                       |           |                                               |
| Provisioning                      | B08 - Get Custom Report                                                     |           |                                               |
| Provisioning                      | B09 - Setting a new NetworkConnectionProfile                                |           |                                               |
| Provisioning                      | B10 - Migrate to new ConnectionProfile                                      |           |                                               |
| Provisioning                      | B11 - Reset - Without Ongoing Transaction                                   |           |                                               |
| Provisioning                      | B12 - Reset - With Ongoing Transaction                                      |           |                                               |
| Authorization                     | C01 - EV Driver Authorization using RFID                                    |           |                                               |
| Authorization                     | C02 - Authorization using a start button                                    |           |                                               |
| Authorization                     | C03 - Authorization using credit/debit card                                 |           |                                               |
| Authorization                     | C04 - Authorization using PIN-code                                          |           |                                               |
| Authorization                     | C05 - Authorization for CSMS initiated transactions                         |           |                                               |
| Authorization                     | C06 - Authorization using local id type                                     |           |                                               |
| Authorization                     | C07 - Authorization using Contract Certificates                             |           |                                               |
| Authorization                     | C08 - Authorization at EVSE using ISO 15118 External Identification Means   |           |                                               |
| Authorization                     | C09 - Authorization by GroupId                                              |           |                                               |
| Authorization                     | C10 - Store Authorization Data in the Authorization Cache                   |           |                                               |
| Authorization                     | C11 - Clear Authorization Data in Authorization Cache                       |           |                                               |
| Authorization                     | C12 - Start Transaction - Cached Id                                         |           |                                               |
| Authorization                     | C13 - Offline Authorization through Local Authorization List                |           |                                               |
| Authorization                     | C14 - Online Authorization through Local Authorization List                 |           |                                               |
| Authorization                     | C15 - Offline Authorization of unknown Id                                   |           |                                               |
| Authorization                     | C16 - Stop Transaction with a Master Pass                                   |           |                                               |
| LocalAuthorizationList Management | D01 - Send Local Authorization List                                         |           |                                               |
| LocalAuthorizationList Management | D02 - Get Local List Version                                                |           |                                               |
| Transactions                      | E01 - Start Transaction options                                             |           |                                               |
| Transactions                      | E02 - Start Transaction - Cable Plugin First                                |           |                                               |
| Transactions                      | E03 - Start Transaction - IdToken First                                     |           |                                               |
| Transactions                      | E04 - Transaction started while Charging Station is offline                 |           |                                               |
| Transactions                      | E05 - Start Transaction - Id not Accepted                                   |           |                                               |
| Transactions                      | E06 - Stop Transaction options                                              |           |                                               |
| Transactions                      | E07 - Transaction locally stopped by IdToken                                |           |                                               |
| Transactions                      | E08 - Transaction stopped while Charging Station is offline                 |           |                                               |
| Transactions                      | E09 - When cable disconnected on EV-side: Stop Transaction                  |           |                                               |
| Transactions                      | E10 - When cable disconnected on EV-side: Suspend Transaction               |           |                                               |
| Transactions                      | E11 - Connection Loss During Transaction                                    |           |                                               |
| Transactions                      | E12 - Inform CSMS of an Offline Occurred Transaction                        |           |                                               |
| Transactions                      | E13 - Transaction-related message not accepted by CSMS                      |           |                                               |
| Transactions                      | E14 - Check transaction status                                              |           |                                               |
| Transactions                      | E15 - End of charging process                                               |           |                                               |
| RemoteControl                     | F01 - Remote Start Transaction - Cable Plugin First                         |           |                                               |
| RemoteControl                     | F02 - Remote Start Transaction - Remote Start First                         | Yes       |                                               |
| RemoteControl                     | F03 - Remote Stop Transaction                                               | Yes       |                                               |
| RemoteControl                     | F04 - Remote Stop ISO 15118 Charging from CSMS                              |           |                                               |
| RemoteControl                     | F05 - Remotely Unlock Connector                                             |           |                                               |
| RemoteControl                     | F06 - Trigger Message                                                       |           |                                               |
| Availability                      | G01 - Status Notification                                                   | Yes       |                                               |
| Availability                      | G02 - Heartbeat                                                             | Yes       |                                               |
| Availability                      | G03 - Change Availability EVSE                                              |           |                                               |
| Availability                      | G04 - Change Availability Charging Station                                  |           |                                               |
| Availability                      | G05 - Lock Failure                                                          |           |                                               |
| Reservation                       | H01 - Reservation                                                           |           |                                               |
| Reservation                       | H02 - Cancel Reservation                                                    |           |                                               |
| Reservation                       | H03 - Use a reserved Connector                                              |           |                                               |
| Reservation                       | H04 - Reservation Ended                                                     |           |                                               |
| TariffAndCost                     | I01 - Show EV Driver-specific Tariff Information                            |           |                                               |
| TariffAndCost                     | I02 - Show EV Driver Running Total Cost During Charging                     |           |                                               |
| TariffAndCost                     | I03 - Show EV Driver Final Total Cost After Charging                        |           |                                               |
| TariffAndCost                     | I04 - Show Fallback Tariff Information                                      |           |                                               |
| TariffAndCost                     | I05 - Show Fallback Total Cost Message                                      |           |                                               |
| TariffAndCost                     | I06 - Update Tariff Information During Transaction                          |           |                                               |
| MeterValues                       | J01 - Sending Meter Values not related to a transaction                     |           |                                               |
| MeterValues                       | J02 - Sending transaction related Meter Values                              |           |                                               |
| MeterValues                       | J03 - Charging Loop with metering information exchange                      |           |                                               |
| SmartCharging                     | K01 - SetChargingProfile                                                    |           |                                               |
| SmartCharging                     | K02 - Central Smart Charging                                                |           |                                               |
| SmartCharging                     | K03 - Local Smart Charging                                                  |           |                                               |
| SmartCharging                     | K04 - Internal Load Balancing                                               |           |                                               |
| SmartCharging                     | K05 - Remote Start Transaction with Charging Profile                        |           |                                               |
| SmartCharging                     | K06 - Offline Behavior Smart Charging During Transaction                    |           |                                               |
| SmartCharging                     | K07 - Offline Behavior Smart Charging at Start of Transaction               |           |                                               |
| SmartCharging                     | K08 - Get Composite Schedule                                                |           |                                               |
| SmartCharging                     | K09 - Get Charging Profiles                                                 |           |                                               |
| SmartCharging                     | K10 - Clear Charging Profile                                                |           |                                               |
| SmartCharging                     | K11 - Set / Update External Charging Limit With Ongoing Transaction         |           |                                               |
| SmartCharging                     | K12 - Set / Update External Charging Limit Without Ongoing Transaction      |           |                                               |
| SmartCharging                     | K13 - Reset / Release External Charging Limit                               |           |                                               |
| SmartCharging                     | K14 - External Charging Limit with Local Controller                         |           |                                               |
| SmartCharging                     | K15 - Charging with load leveling based on High Level Communication         |           |                                               |
| SmartCharging                     | K16 - Optimized charging with scheduling to the CSMS                        |           |                                               |
| SmartCharging                     | K17 - Renegotiating a Charging Schedule                                     |           |                                               |
| FirmwareManagement                | L01 - Secure Firmware Update                                                |           |                                               |
| FirmwareManagement                | L02 - Non-Secure Firmware Update                                            |           |                                               |
| FirmwareManagement                | L03 - Publish Firmware file on Local Controller                             |           |                                               |
| FirmwareManagement                | L04 - Unpublish Firmware file on Local Controller                           |           |                                               |
| ISO 15118 CertificateManagement   | M01 - Certificate installation EV                                           |           |                                               |
| ISO 15118 CertificateManagement   | M02 - Certificate Update EV                                                 |           |                                               |
| ISO 15118 CertificateManagement   | M03 - Retrieve list of available certificates from a Charging Station       |           |                                               |
| ISO 15118 CertificateManagement   | M04 - Delete a specific certificate from a Charging Station                 |           |                                               |
| ISO 15118 CertificateManagement   | M05 - Install CA certificate in a Charging Station                          |           |                                               |
| ISO 15118 CertificateManagement   | M06 - Get Charging Station Certificate status                               |           |                                               |
| Diagnostics                       | N01 - Retrieve Log Information                                              |           |                                               |
| Diagnostics                       | N02 - Get Monitoring report                                                 |           |                                               |
| Diagnostics                       | N03 - Set Monitoring Base                                                   |           |                                               |
| Diagnostics                       | N04 - Set Variable Monitoring                                               |           |                                               |
| Diagnostics                       | N05 - Set Monitoring Level                                                  |           |                                               |
| Diagnostics                       | N06 - Clear / Remove Monitoring                                             |           |                                               |
| Diagnostics                       | N07 - Alert Event                                                           |           |                                               |
| Diagnostics                       | N08 - Periodic Event                                                        |           |                                               |
| Diagnostics                       | N09 - Get Customer Information                                              |           |                                               |
| Diagnostics                       | N10 - Clear Customer Information                                            |           |                                               |
| DisplayMessage                    | O01 - Set DisplayMessage                                                    |           |                                               |
| DisplayMessage                    | O02 - Set DisplayMessage for Transaction                                    |           |                                               |
| DisplayMessage                    | O03 - Get All DisplayMessages                                               |           |                                               |
| DisplayMessage                    | O04 - Get Specific DisplayMessages                                          |           |                                               |
| DisplayMessage                    | O05 - Clear a DisplayMessage                                                |           |                                               |
| DisplayMessage                    | O06 - Replace DisplayMessage                                                |           |                                               |
| DataTransfer                      | P01 - Data Transfer to the Charging Station                                 |           |                                               |
| DataTransfer                      | P02 - Data Transfer to the CSMS                                             |           |                                               |
