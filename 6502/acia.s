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
  .export blocking_transmit
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
  lda #$FF
  sta ACIA_TX_BUFFER_WRITE_PTR
  sta ACIA_TX_BUFFER_READ_PTR
  sta ACIA_RX_BUFFER_WRITE_PTR
  sta ACIA_RX_BUFFER_READ_PTR

  stz ACIA_TX_IN_PROGRESS

  ; Init rx buffer to all FF for easier testing
  ldy #0
  lda #$FF
@loop_rx:
  sta ACIA_RX_BUFFER, Y
  iny
  bne @loop_rx

  ; Same but tx
  ldy #0
  lda #$FE
@loop_tx:
  sta ACIA_TX_BUFFER, Y
  iny
  bne @loop_tx

  ; 1 stop bit, 8 bits, rcv baud rate, 9600 on crystal
  lda #%00011110
  ; 1 stop bit, 8 bits, rcv baud rate, 600 on crystal
  lda #%00010111
  sta ACIA_CONTROL_REGISTER
  ; No parity, normal mode, RTSB low, no tx interrupt, rx interrupt, data terminal ready (unused)
  lda #%11001001
  sta ACIA_COMMAND_REGISTER

  jsr via_prep_for_transmit
  rts

; Will start the transmit on the next T2 tick
initiate_transmit:
  bit ACIA_TX_IN_PROGRESS
  bmi @done
  dec ACIA_TX_IN_PROGRESS
  ; Start T2 by writing to the high byte
  pha
  lda #TX_T2_PULSES
  sta VIA_T2CL
  stz VIA_T2CH
  pla
@done:
  rts

blocking_transmit:
  jsr initiate_transmit
@block:
  bit ACIA_TX_IN_PROGRESS
  bmi @block
  rts


; Clobbers Y
; Return a value instead of just initiating transmit?
write_transmit_byte:
  pha
  ; Check if buffer full
  lda ACIA_TX_BUFFER_WRITE_PTR
  inc
  cmp ACIA_TX_BUFFER_READ_PTR
  bne @ready
  ; Buffer full, initiate transmit and wait a bit
  jsr initiate_transmit
@wait:
  wai
  cmp ACIA_TX_BUFFER_READ_PTR
  bne @wait
@ready:
  tay
  pla
  sty ACIA_TX_BUFFER_WRITE_PTR
  sta ACIA_TX_BUFFER, Y
  rts

; Called from NMI
; TODO set RTS line to prevent overflow
; TODO handle overrun n stuff
acia_receive:
  lda ACIA_DATA_REGISTERS
  inc ACIA_RX_BUFFER_WRITE_PTR
  ldy ACIA_RX_BUFFER_WRITE_PTR
  sta ACIA_RX_BUFFER, Y
  rts


; Called from IRQ
; Ignore CTS, we cannot read the line directly and transmit status is stuck on
; TODO can AND CTS with PB6?
acia_transmit:
  ; Check if buffer empty
  lda ACIA_TX_BUFFER_WRITE_PTR
  cmp ACIA_TX_BUFFER_READ_PTR
  beq @empty
  ; If not, send a byte and reinit T2
  inc ACIA_TX_BUFFER_READ_PTR
  ldy ACIA_TX_BUFFER_READ_PTR
  lda ACIA_TX_BUFFER, Y
  sta ACIA_DATA_REGISTERS
  stz VIA_T2CH
  bra @done
@empty:
  stz ACIA_TX_IN_PROGRESS
@done:
  rts
