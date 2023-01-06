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

| Start frame delimiter | Data                                  | CRC    | End frame delimiter |
|-----------------------|---------------------------------------|--------|---------------------|
| 1 byte                | Up to 196 bytes (TODO set better max) | 1 byte | 1 byte              |

* Start delimiter = end delimiter = 0x21 ('!')
* Escape = 0x23 ('#')
* CRC: 0x00 until I figure out how to do this properly

### Payload

Data format:

| Type   | Payload         |
|--------|-----------------|
| 1 byte | Up to 195 bytes |

### Messages

| Type        | Payload                |
|-------------|------------------------|
| Show string | Null-terminated string |
