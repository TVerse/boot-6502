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

  .import __ACIA_START__
  .import ACIA_TX_BUFFER_WRITE_PTR
  .import ACIA_TX_BUFFER_READ_PTR
  .import ACIA_RX_BUFFER_WRITE_PTR
  .import ACIA_RX_BUFFER_READ_PTR
  .import ACIA_TX_IN_PROGRESS
  .import ACIA_TX_BUFFER
  .import ACIA_RX_BUFFER
  .import via_prep_for_transmit
  .import VIA_T2CL
  .import VIA_T2CH

  .export ACIA_STATUS_RESET_REGISTERS
  .export acia_receive
  .export acia_transmit
  .export init_acia
  .export block_transmit
  .export initiate_transmit
  .export write_transmit_byte

ACIA_DATA_REGISTERS = __ACIA_START__ + $00
ACIA_STATUS_RESET_REGISTERS = __ACIA_START__ + $01
ACIA_COMMAND_REGISTER = __ACIA_START__ + $02
ACIA_CONTROL_REGISTER = __ACIA_START__ + $03

; 10 symbols * 16 counts/symbol
TX_T2_PULSES = 255 ; 160 breaks in a weird way (on memcpy?) No further optimization

init_acia:
  ; Set buffer pointers
  LDA #$FF
  STA ACIA_TX_BUFFER_WRITE_PTR
  STA ACIA_TX_BUFFER_READ_PTR
  STA ACIA_RX_BUFFER_WRITE_PTR
  STA ACIA_RX_BUFFER_READ_PTR

  STZ ACIA_TX_IN_PROGRESS

  ; Init rx buffer to all FF for easier testing
  LDY #0
  LDA #$FF
@loop_rx:
  STA ACIA_RX_BUFFER, Y
  INY
  BNE @loop_rx

  ; Same but tx
  LDY #0
  LDA #$FE
@loop_tx:
  STA ACIA_TX_BUFFER, Y
  INY
  BNE @loop_tx

  ; 1 stop bit, 8 bits, rcv baud rate, 9600 on crystal
  LDA #%00011110
  ; 1 stop bit, 8 bits, rcv baud rate, 600 on crystal
  LDA #%00010111
  STA ACIA_CONTROL_REGISTER
  ; No parity, normal mode, RTSB low, no tx interrupt, rx interrupt, data terminal ready (unused)
  LDA #%11001001
  STA ACIA_COMMAND_REGISTER

  JSR via_prep_for_transmit
  RTS

; Will start the transmit on the next T2 tick
initiate_transmit:
  BIT ACIA_TX_IN_PROGRESS
  BMI @done
  DEC ACIA_TX_IN_PROGRESS
  ; Start T2 by writing to the high byte
  PHA
  LDA #TX_T2_PULSES
  STA VIA_T2CL
  STZ VIA_T2CH
  PLA
@done:
  RTS

block_transmit:
  JSR initiate_transmit
@block:
  BIT ACIA_TX_IN_PROGRESS
  BMI @block
  RTS


; Clobbers Y
; Return a value instead of just initiating transmit?
write_transmit_byte:
  PHA
  ; Check if buffer full
  LDA ACIA_TX_BUFFER_WRITE_PTR
  INC
  CMP ACIA_TX_BUFFER_READ_PTR
  BNE @ready
  ; Buffer full, initiate transmit and wait a bit
  JSR initiate_transmit
@wait:
  WAI
  CMP ACIA_TX_BUFFER_READ_PTR
  BNE @wait
@ready:
  TAY
  PLA
  STY ACIA_TX_BUFFER_WRITE_PTR
  STA ACIA_TX_BUFFER, Y
  RTS

; Called from NMI
; TODO set RTS line to prevent overflow
; TODO handle overrun n stuff
acia_receive:
  LDA ACIA_DATA_REGISTERS
  INC ACIA_RX_BUFFER_WRITE_PTR
  LDY ACIA_RX_BUFFER_WRITE_PTR
  STA ACIA_RX_BUFFER, Y
  RTS


; Called from IRQ
; Ignore CTS, we cannot read the line directly and transmit status is stuck on
; TODO can AND CTS with PB6?
acia_transmit:
  ; Check if buffer empty
  LDA ACIA_TX_BUFFER_WRITE_PTR
  CMP ACIA_TX_BUFFER_READ_PTR
  BEQ @empty
  ; If not, send a byte and reinit T2
  INC ACIA_TX_BUFFER_READ_PTR
  LDY ACIA_TX_BUFFER_READ_PTR
  LDA ACIA_TX_BUFFER, Y
  STA ACIA_DATA_REGISTERS
  STZ VIA_T2CH
  BRA @done
@empty:
  STZ ACIA_TX_IN_PROGRESS
@done:
  RTS
