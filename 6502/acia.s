; Considerations:
; * Control RTS based on buffer size (need to be a bit early to account for in-transit data!)
; * Hook up to NMI
; * NMI is the only thing that writes the rx buffer
; * Do tx from IRQ with T2/PB6 count cause bug
;   * Transmit bit in status register is stuck on, irq constantly triggers and can't poll it either.
; * At 115200 baud we have 86 cycles @ 1MHz to finish up, but can't go that fast due to transmit bug.
;   * To get good counts we can't be faster than PIH2 / 4 == 250KHz, 9600 -> 153.6KHz (16 counts per symbol).
;
; Is it possible to make the NMI overlap-safe?

  .ifndef ACIA_BASE
ACIA_BASE = $5000
  .endif

ACIA_DATA_REGISTERS = ACIA_BASE + $0
ACIA_STATUS_RESET_REGISTERS = ACIA_BASE + $1
ACIA_COMMAND_REGISTER = ACIA_BASE + $2
ACIA_CONTROL_REGISTER = ACIA_BASE + $3

TX_T2_PULSES = 500 ; TODO increase this a bit for leeway?
TX_T2_L = <TX_T2_PULSES
TX_T2_H = >TX_T2_PULSES

init_acia:
  ; Set buffer pointers
  STZ acia_tx_buffer_write_ptr
  STZ acia_tx_buffer_read_ptr
  STZ acia_rx_buffer_write_ptr
  STZ acia_rx_buffer_read_ptr
  STZ acia_tx_in_progress

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
  DEC acia_tx_in_progress
  ; Start T2 by writing to the high byte
  PHA
  LDA #TX_T2_L
  STA VIA_T2CL
  LDA #TX_T2_H
  STA VIA_T2CH
  PLA
.done:
  RTS

; Clobbers Y
; Return a value instead of just initiating transmit?
write_transmit_byte:
  PHA
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
  DEY
  PLA
  STA acia_tx_buffer, Y
  INC acia_tx_buffer_write_ptr
  RTS

; Called from IRQ
; Ignore CTS, we cannot read the line directly and transmit status is stuck on
transmit:
  ; Check if buffer empty
  LDA acia_tx_buffer_write_ptr
  INC
  CMP acia_tx_buffer_read_ptr
  BEQ .empty
  ; If not, send a byte and reinit T2
  LDY acia_tx_buffer_read_ptr
  LDA acia_tx_buffer, Y
  INC acia_tx_buffer_read_ptr
  STA ACIA_DATA_REGISTERS
  LDA #TX_T2_H
  STA VIA_T2CH
  BRA .done
.empty:
  LDA VIA_T1CL
  STZ acia_tx_in_progress
.done:
  RTS
