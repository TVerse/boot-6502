; Considerations:
; * Control RTS based on buffer size (need to be a bit early to account for in-transit data!)
; * Use VIA T1/PB7 until crystal comes in
; * Hook up to NMI
; * NMI is the only thing that writes the rx buffer and reads the tx buffer (so no catchup after)
;   * Is that possible? How to initiate writes without having to keep interrupts on?
; * At 115200 baud we have 86 cycles @ 1MHz to finish up.
;
; Is it possible to make the NMI overlap-safe?

  .macro INIT_ACIA
  ; Set pointers
  STZ acia_tx_buffer_write_ptr
  STZ acia_tx_buffer_read_ptr
  STZ acia_xx_buffer_write_ptr
  STZ acia_tx_buffer_read_ptr
  .endmacro

  .macro SEND_BYTE
  LDY acia_tx_buffer_read_ptr
  INC acia_tx_buffer_read_ptr
  LDA acia_tx_buffer, Y
  STA ACIA_DATA_REGISTERS
  .endmacro

  .macro RECEIVE_BYTE
  LDY acia_rx_buffer_write_ptr
  INC acia_rx_buffer_write_ptr
  LDA ACIA_DATA_REGISTERS
  STA acia_rx_buffer, Y
  .endmacro
