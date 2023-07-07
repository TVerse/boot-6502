; Considerations:
; * Control RTS based on buffer size (need to be a bit early to account for in-transit data!)
; * Hook up to NMI
; * NMI is the only thing that writes the rx buffer
; * Do tx from IRQ with T2/PB6 count cause bug
;   * Transmit bit in status register is stuck on, irq constantly triggers and can't poll it either.
; * At 115200 baud we have 86 cycles @ 1MHz to finish up, but can't go that fast due to transmit bug.
;   * To get good counts we can't be faster than PIH2 / 4 == 250KHz, 9600 -> 153.6KHz (16 counts per symbol).
; * TODO can I replace counting with RC + Schmitt trigger on tx line?
;
; Is it possible to make the NMI overlap-safe?

.include "via.inc"
.include "stack.inc"
.include "error.inc"

.import __ACIA_START__

.export acia_init
.export acia_transmit_byte
.export acia_receive_byte
.export acia_parse_buffer
.export acia_block_handle_message
.export ACIA_STATUS

.enum StopBits
One= %00000000
Other= %10000000
.endenum

.enum WordLength
Eight = %00000000
Seven = %00100000
Six = %01000000
Five = %01100000
.endenum

.enum ClockSource
External = %00000000
BaudRateGenerator = %00010000
.endenum

.enum BaudRate
ExternalX16 = 0
B50 = 1
B75 = 2
B109 = 3
B134 = 4
B150 = 5
B300 = 6
B600 = 7
B1200 = 8
B1800 = 9
B2400 = 10
B3600 = 11
B4800 = 12
B7200 = 13
B9600 = 14
B19200 = 15
.endenum

.enum ParityCheck
NoParity = %00000000
OddBoth = %00100000
EvenBoth = %01100000
MarkTransmitNoCheck = %10100000
SpaceTransmitNoCheck = %11100000
.endenum

.enum EchoMode
Normal = %00000000
Echo = %00010000
.endenum

.enum TransmitControls
NoInterruptRTSbHigh = %00000000
; InterruptEnabledRTSLow = %00000100 ; TX interrupts broken!
NoInterruptRTSbLow = %00001000
NoInterruptRTSbLowTransmitBreak = %00001100
.endenum

.enum ReceiverInterrupt
Enable = %00000000
Disable = %00000010
.endenum

.enum DataTerminalReady
Disable = %00000000
Enable = %00000001
.endenum

.enum TransmissionState
Idle = 0
Primed = 1
StartByteSent = 2
Sending = 3
Escaped = 4
EndByteSent = 5
.endenum

.enum ReceiveDecodeState
Waiting = 0
WaitingForTag = 1
Accepted = 1
Escaped = 2
Done = 3
.endenum

ACIA_DATA = __ACIA_START__ + $00
ACIA_STATUS = __ACIA_START__ + $01
ACIA_COMMAND = __ACIA_START__ + $02
ACIA_CONTROL = __ACIA_START__ + $03

; 10 symbols * 16 counts/symbol
TX_T2_PULSES = 250                  ; 160 breaks in a weird way (on memcpy?) No further optimization
START_BYTE = '('
END_BYTE = ')'
ESCAPE_BYTE = $5C ; \
ESCAPED_XOR = $20

.enum FrameType
Echo = $01
Echoed = $02
.endenum

.data
; One byte with page-aligned buffers. (TODO not needed with indexed addressing?)
; Both pointers are use-then-increment (so pointing past the last byte read/written)
; Buffer is full if (write + 1) == read
; Buffer is empty if write == read
; If these are full pointers instead of an increment, does addressing become simpler?
; Might free up a register. But addition doesn't auto-modulo anymore so align to page or do 16-bit add.
ACIA_TRANSMISSION_STATE: .byte TransmissionState::Idle
ACIA_RECEIVE_DECODE_STATE: .byte ReceiveDecodeState::Waiting
ACIA_RX_BUFFER_WRITE_IDX: .byte $00
ACIA_RX_BUFFER_READ_IDX: .byte $00
ACIA_RX_FRAME_WRITE_IDX: .byte $00
ACIA_RX_FRAME_LEN: .byte $00
ACIA_TX_FRAME_READ_IDX: .byte $00
ACIA_TX_FRAME_WRITE_IDX: .byte $00

; X is needed for jump, so handlers *must* pull it off the stack
ACIA_PAYLOAD_HANDLERS:
Echo: .word _acia_echo_handler
Echoed: .word _acia_null_handler

.bss
RX_TYPE: .res 1
TX_TYPE: .res 1

.segment "BUFFERS"
.align $0100
ACIA_RX_BUFFER: .res $0100
ACIA_RX_FRAME: .res $0100
ACIA_TX_FRAME: .res $0100

.rodata
tx_buffer_full_msg: .asciiz "TX buffer full"
unknown_transmission_state_msg: .asciiz "Unknown TX state"
unknown_receive_decode_state_msg: .asciiz "Unknown RX state"
receive_buffer_overflow: .asciiz "Receive buffer overflow"

.code

.macro tx_block_until_done
.local loop
loop:
    lda ACIA_TRANSMISSION_STATE
    wai
    bne loop
.endmacro

.macro tx_frame_checks
.local done
    pha
    ; Block when transmit is already in progress
    tx_block_until_done
    ; Verify room in buffer
    lda ACIA_TX_FRAME_WRITE_IDX
    bne done
    literal tx_buffer_full_msg
    jmp error
done:
    pla
.endmacro

; TODO constructor
acia_init:
  ; Init rx buffer to all FF for easier testing
    ldy #0
    lda #$FF
@loop_rx:
    sta ACIA_RX_BUFFER, Y
    iny
    bne @loop_rx

  ; Same but frames
    ldy #0
    lda #$FF
@loop_tx:
    sta ACIA_RX_FRAME, Y
    sta ACIA_TX_FRAME, Y
    iny
    bne @loop_tx

    lda #(StopBits::One | WordLength::Eight | ClockSource::BaudRateGenerator | BaudRate::B600)
    sta ACIA_CONTROL
    lda #(ParityCheck::NoParity | EchoMode::Normal | TransmitControls::NoInterruptRTSbLow | ReceiverInterrupt::Enable | DataTerminalReady::Disable)
    sta ACIA_COMMAND

    jsr via_prep_for_transmit
    rts

; No use, no clobber
acia_new_transmit:
    ; Could probably also just allow writes as long as they are behind the write ptr (or transmit is done)
    tx_block_until_done
    stz ACIA_TX_FRAME_WRITE_IDX
    rts

; Uses A, no clobber
acia_add_byte_for_transmit:
    tx_frame_checks
    phy
    ldy ACIA_TX_FRAME_WRITE_IDX
    sta ACIA_TX_FRAME, Y
    inc ACIA_TX_FRAME_WRITE_IDX
    ply
    rts

; Clobbers A
acia_start_transmit:
    tx_block_until_done
    lda #TransmissionState::Primed
    sta ACIA_TRANSMISSION_STATE
    lda #TX_T2_PULSES
    sta VIA_T2CL
    stz VIA_T2CH
    rts

acia_transmit_byte:
    ; TODO try a branch table?
    lda ACIA_TRANSMISSION_STATE
    beq @return
    cmp #TransmissionState::Primed
    beq @primed
    cmp #TransmissionState::StartByteSent
    beq @startbytesent
    cmp #TransmissionState::Sending
    beq @sending
    cmp #TransmissionState::Escaped
    beq @escaped
    cmp #TransmissionState::EndByteSent
    beq @endbytesent
    ; Should be impossible!
    literal unknown_transmission_state_msg
    jmp error
    ; Not at the bottom cause out of branch range
@return:
    rts
@primed:
    stz ACIA_TX_FRAME_READ_IDX
    lda #START_BYTE
    sta ACIA_DATA
    lda #TransmissionState::StartByteSent
    sta ACIA_TRANSMISSION_STATE
    bra @start_timer
@startbytesent:
    lda TX_TYPE
    sta ACIA_DATA
    lda #TransmissionState::Sending
    sta ACIA_TRANSMISSION_STATE
    bra @start_timer
@sending:
    ldy ACIA_TX_FRAME_READ_IDX
    cmp ACIA_TX_FRAME_WRITE_IDX
    beq @done
    lda ACIA_TX_FRAME, Y
    cmp #START_BYTE
    beq @start_escape
    cmp #END_BYTE
    beq @start_escape
    cmp #ESCAPE_BYTE
    beq @start_escape
    sta ACIA_DATA
    bra @start_timer
@start_escape:
    lda ESCAPE_BYTE
    sta ACIA_DATA
    lda #TransmissionState::Escaped
    sta ACIA_TRANSMISSION_STATE
    bra @start_timer
@escaped:
    ldy ACIA_TX_FRAME_READ_IDX
    lda ACIA_TX_FRAME, Y
    eor #ESCAPED_XOR
    sta ACIA_DATA
    lda #TransmissionState::Sending
    sta ACIA_TRANSMISSION_STATE
    bra @start_timer
@done:
    lda #END_BYTE
    sta ACIA_DATA
    lda #TransmissionState::EndByteSent
    bra @start_timer
@endbytesent:
    lda #TransmissionState::Idle
    sta ACIA_TRANSMISSION_STATE
    bra @return
@start_timer:
    stz VIA_T2CH
    bra @return

acia_receive_byte:
    ldy ACIA_RX_BUFFER_WRITE_IDX
    iny
    cpy ACIA_RX_BUFFER_READ_IDX
    beq @buffer_overflow
    lda ACIA_DATA
    sta ACIA_RX_BUFFER, Y
    sty ACIA_RX_BUFFER_WRITE_IDX
    ; TODO manage RTSb
    rts
@buffer_overflow:
    literal receive_buffer_overflow
    jmp error

acia_parse_buffer:
    ldy ACIA_RX_BUFFER_READ_IDX
    cpy ACIA_RX_BUFFER_WRITE_IDX
    ; Buffer empty, done
    beq @return
    inc ACIA_RX_BUFFER_READ_IDX
    lda ACIA_RECEIVE_DECODE_STATE
    cpy #ReceiveDecodeState::Waiting
    beq @waiting
    cpy #ReceiveDecodeState::WaitingForTag
    beq @waiting_for_tag
    cpy #ReceiveDecodeState::Accepted
    beq @accepted
    cpy #ReceiveDecodeState::Escaped
    beq @escaped
    cpy #ReceiveDecodeState::Done
    beq @return
    literal unknown_receive_decode_state_msg
    jmp error
@return:
    rts
@waiting:
    lda ACIA_RX_BUFFER, Y
    cmp #START_BYTE
    beq @start_byte_found
    jmp acia_parse_buffer
@start_byte_found:
    stz ACIA_RX_FRAME_WRITE_IDX
    lda #ReceiveDecodeState::WaitingForTag
    sta ACIA_RECEIVE_DECODE_STATE
    jmp acia_parse_buffer
@waiting_for_tag:
    ; Grab tag type
    lda ACIA_RX_BUFFER, Y
    sta RX_TYPE
    lda #ReceiveDecodeState::Accepted
    sta ACIA_RECEIVE_DECODE_STATE
    jmp acia_parse_buffer
@accepted:
    lda ACIA_RX_BUFFER, Y
    cmp #ESCAPE_BYTE
    beq @start_escape
    cmp #START_BYTE
    beq @start_byte_found
    cmp #END_BYTE
    beq @end_byte_found
    ldy ACIA_RX_FRAME_WRITE_IDX
    sta ACIA_RX_FRAME, Y
    inc ACIA_RX_FRAME_WRITE_IDX
    jmp acia_parse_buffer
@start_escape:
    lda #ReceiveDecodeState::Escaped
    sta ACIA_RECEIVE_DECODE_STATE
    jmp acia_parse_buffer
@end_byte_found:
    sty ACIA_RX_FRAME_LEN
    lda #ReceiveDecodeState::Done
    sta ACIA_RECEIVE_DECODE_STATE
    jmp acia_parse_buffer
@escaped:
    ; Decode escaped char
    lda ACIA_RX_BUFFER, Y
    eor #ESCAPED_XOR
    ldy ACIA_RX_FRAME_WRITE_IDX
    sta ACIA_RX_FRAME, Y
    inc ACIA_RX_FRAME_WRITE_IDX
    jmp acia_parse_buffer

acia_mark_message_handled:
    lda #ReceiveDecodeState::Waiting
    sta ACIA_RECEIVE_DECODE_STATE
    rts

acia_block_handle_message:
@block:
    lda ACIA_RECEIVE_DECODE_STATE
    cmp #ReceiveDecodeState::Done
    bne @block
    ; RX_TYPE x 2 since addresses are words
    lda RX_TYPE
    clc
    adc RX_TYPE
    phx
    tax
    phy
    jsr _acia_jump_to_handler
    ply
    jsr acia_mark_message_handled
    rts

_acia_jump_to_handler:
    jmp (ACIA_PAYLOAD_HANDLERS, X)

_acia_null_handler:
    plx
    rts

_acia_echo_handler:
    plx
    jsr acia_new_transmit
    ldy #0
@loop:
    lda ACIA_RX_FRAME, Y
    jsr acia_add_byte_for_transmit
    iny
    cpy ACIA_RX_FRAME_LEN
    bne @loop
    rts
