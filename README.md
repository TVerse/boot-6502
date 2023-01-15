# Boot-6502

Code for the 6502 and Rpi.

Goal is to have some bootcode for the 6502 and a program loader from the Rpi.
Also want to include unit test capabilities.

## 6502 memory map

0x0000-0x3FFF RAM
0x6000-0x600F 6522 VIA
0x5000-0x5003 6551 ACIA
0x8000-0xFFFF ROM

(might increase RAM size)

## Communication protocol.

UART via 6551.

### Framing

1. Start byte '('
2. Data
3. CRC (currently always 0x00)
4. End byte ')'

Escape is '\', escaped byte is XORed with 0x20.

### Payload

Data format:

| Type   | Payload         |
|--------|-----------------|
| 1 byte | Up to 256 bytes |

### Messages

| Type   | Payload         |
|--------|-----------------|
| Echo   | Any binary data |
| Echoed | Any binary data |

### Setup on 6502

* NMI receive
* IRQ + pulse count transmit (so still async)
* Remove the circular buffers for single-frame buffers
* Do unescaping right when the message comes in/gets send
* Max unescaped size: type byte + 256 bytes data
* Set RTSb high after frame received
  * Is that on time if multiple frames happen at once? Still need circular RX buffer for initial stage?

Time to use .data section for some ACIA structs?