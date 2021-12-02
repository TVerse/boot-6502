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

  .ifndef ACIA_BASE
ACIA_BASE = $5000
  .endif

ACIA_DATA_REGISTERS = ACIA_BASE + $0
ACIA_STATUS_RESET_REGISTERS = ACIA_BASE + $1
ACIA_COMMAND_REGISTER = ACIA_BASE + $2
ACIA_CONTROL_REGISTER = ACIA_BASE + $3

TX_T2_PULSES = 255 ; 160 breaks in a weird way (on memcpy?) No further optimization

init_acia:
  ; Set buffer pointers
  LDA #$FF
  STA acia_tx_buffer_write_ptr
  STA acia_tx_buffer_read_ptr
  STA acia_rx_buffer_write_ptr
  STA acia_rx_buffer_read_ptr

  STZ acia_tx_in_progress

  ; Init rx buffer to all FF for easier testing
  LDY #0
  LDA #$FF
.loop_rx
  STA acia_rx_buffer
  INY
  BNE .loop_rx

  ; Same but tx
  LDY #0
  LDA #$FE
.loop_tx:
  STA acia_tx_buffer
  INY
  BNE .loop_tx

  ; 1 stop bit, 8 bits, rcv baud rate, 9600 on crystal
  LDA #%00011110
  STA ACIA_CONTROL_REGISTER
  ; No parity, normal mode, RTSB low, no tx interrupt, rx interrupt, data terminal ready (unused)
  LDA #%11001001
  STA ACIA_COMMAND_REGISTER

  JSR via_prep_for_transmit
  RTS

; Will start the transmit on the next T2 tick
initiate_transmit:
  BIT acia_tx_in_progress
  BMI .done
  INC VIA_PORTA
  DEC acia_tx_in_progress
  ; Start T2 by writing to the high byte
  PHA
  LDA #TX_T2_PULSES
  STA VIA_T2CL
  STZ VIA_T2CH
  PLA
.done:
  RTS

; Clobbers Y
; Return a value instead of just initiating transmit?
write_transmit_byte:
  PHA
  ; Check if buffer full
  LDA acia_tx_buffer_write_ptr
  INC
  CMP acia_tx_buffer_read_ptr
  BNE .ready
  ; Buffer full, initiate transmit and wait a bit
  JSR initiate_transmit
.wait
  WAI
  CMP acia_tx_buffer_read_ptr
  BNE .wait
.ready:
  TAY
  PLA
  STY acia_tx_buffer_write_ptr
  STA acia_tx_buffer, Y
  RTS

; Called from NMI
; TODO set RTS line to prevent overflow
; TODO handle overrun n stuff
acia_receive:
  LDA ACIA_DATA_REGISTERS
  INC acia_rx_buffer_write_ptr
  LDY acia_rx_buffer_write_ptr
  STA acia_rx_buffer, Y
  RTS


; Called from IRQ
; Ignore CTS, we cannot read the line directly and transmit status is stuck on
; TODO can AND CTS with PB6?
acia_transmit:
  ; Check if buffer empty
  LDA acia_tx_buffer_write_ptr
  CMP acia_tx_buffer_read_ptr
  BEQ .empty
  ; If not, send a byte and reinit T2
  INC acia_tx_buffer_read_ptr
  LDY acia_tx_buffer_read_ptr
  LDA acia_tx_buffer, Y
  STA ACIA_DATA_REGISTERS
  STZ VIA_T2CH
  BRA .done
.empty:
  STZ acia_tx_in_progress
.done:
  RTS
