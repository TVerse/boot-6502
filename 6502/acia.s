; Considerations:
; * Control RTS based on buffer size (need to be a bit early to account for in-transit data!)
; * Use VIA T1/PB7 until crystal comes in
; * Hook up to NMI
; * NMI is the only thing that writes the rx buffer and reads the tx buffer (so no catchup after)
;   * Is that possible? How to initiate writes without having to keep interrupts on?
; * At 115200 baud we have 86 cycles @ 1MHz to finish up, but can't go that fast due to transmit bug.
;
; Is it possible to make the NMI overlap-safe?

; PLAN
; 9600 baud with crystal
; RxD pin to PB6 (need to reorder LCD)
; Use VIA T2 for transmit interrupts, 16x cycles, start + stop + 8bit (no parity) = 160. Need leeway?

  .ifndef ACIA_BASE
ACIA_BASE = $5000
  .endif

ACIA_DATA_REGISTERS = ACIA_BASE + $0
ACIA_STATUS_RESET_REGISTERS = ACIA_BASE + $1
ACIA_COMMAND_REGISTER = ACIA_BASE + $2
ACIA_CONTROL_REGISTER = ACIA_BASE + $3

TX_T2_PULSES = 160 ; TODO increase this a bit for leeway?

init_acia:
  ; Set buffer pointers
  STZ acia_tx_buffer_write_ptr
  STZ acia_tx_buffer_read_ptr
  STZ acia_rx_buffer_write_ptr
  STZ acia_rx_buffer_read_ptr

  ; 1 stop bit, 8 bits, rcv baud rate, 9600 on crystal
  LDA #%00011110
  STA ACIA_CONTROL_REGISTER
  ; No parity, normal mode, RTSB high, no tx interrupt, rx interrupt, data terminal ready (unused)
  LDA #%11000001
  STA ACIA_COMMAND_REGISTER

  ; Prepare VIA
  LDA TX_T2_PULSES
  JSR via_prep_for_transmit
  RTS

; Will start the transmit on the next T2 tick
initiate_transmit:
  ; If T2 low is zero we're not transmitting anything.
  ; Or we got really unlucky and an interrupt is about to trigger. Both are fine.
  BIT acia_tx_in_progress
  BPL .done
  DEC acia_tx_in_progress
  ; Start T2 by writing to the high byte
  STZ VIA_T2CH
  ; Pull RTS low
  PHA
  LDA ACIA_COMMAND_REGISTER
  EOR #%00001000
  STA ACIA_COMMAND_REGISTER
  PLA
.done:
  RTS

; Clobbers Y
write_transmit_byte:
  PHA
  BIT acia_tx_in_progress
  BNE .ready
  PHA
  JSR initiate_transmit
  PLA
.wait
  WAI
  CMP acia_tx_buffer_read_ptr
  BNE .wait
.ready:
  TAY
  PLA
  STA acia_tx_buffer_write_ptr, Y
  INC acia_tx_buffer_write_ptr
  RTS

; Called from IRQ
transmit:
  ; Check if buffer empty
  LDA acia_tx_buffer_write_ptr
  CMP acia_tx_buffer_read_ptr
  BEQ .empty
  ; If not, send a byte and reinit T2
  LDY acia_tx_buffer_read_ptr
  LDA acia_tx_buffer_read_ptr, Y
  STA ACIA_DATA_REGISTERS
  STZ VIA_T2CH
  BRA .done
.empty:
  STZ acia_tx_in_progress
.done:
  RTS