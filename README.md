# Boot-6502

Code for the 6502 and Arduino.

Goal is to have some bootcode for the 6502 and a program loader from the Arduino.
Also want to include unit test capabilities.

## 6502 memory map
0x0000-0x3FFF RAM
0x6000-0x600F 6522 VIA
0x8000-0xFFFF ROM

(might increase RAM size)

## Communication protocol.

8 bit parallel port, 2-way handshake using the 6522 VIA.

Arduino always initiates (TODO is that good?) and will reset the 6502 when it gets reset.

Response format:
* ACK: 0x01
* ACKDATA: 0x02

Registers:
* A: 0x01
* X: 0x02
* Y: 0x03

Lengths are nonzero, 0 is interpreted as 256.

| Name | Command byte | Other bytes | Response format | | Max request length | Max response length |
| --- | --- | --- | --- | --- | --- |
| Display string | 0xFF | LEN DATA | ACK | | 258 | 1 |
| Write bytes | 0x01 | ADDR LEN DATA | ACK | 260 | 1 |
| Read bytes | 0x02 | ADDR LEN | ACKDATA DATA| 4 | 257 |
| Read register | 0x03 | REGISTER | ACKDATA VALUE | 2 | 2 |
| JMP A | 0x04 | ADDR | ACK | 3 | 1 |
| JSR A | 0x05 | ADDR | ACK | 3 | 1 |

Q: ACK before or after return for JSR?

Q: Display String is overkill, can also do it with write/JSR.
