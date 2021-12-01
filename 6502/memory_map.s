ROM_START_ADDR = $8000
VECTORS_START_ADDR = $FFFA

VIA_BASE = $6000

ACIA_BASE = $5000

SOFTWARE_STACK_START = $F5 ; Grows down

N = SOFTWARE_STACK_START + 1 ; 8 bytes
N_IRQ = N + 8; 2 bytes

program_nmi = $3FFA
program_reset = $3FFC
program_irq = $3FFE

initialization_done = $0200 ; 1 byte
; 1 byte free
ten_millisecond_counter_addr = $0202 ; 2 bytes

; One byte with page-aligned buffers.
; Both pointers are increment-then-use
; Buffer is full if (write + 1) == read
; Buffer is empty if write == read
acia_tx_in_progress = $02FB
acia_tx_buffer_write_ptr = $02FC
acia_tx_buffer_read_ptr = $02FD
acia_rx_buffer_write_ptr = $02FE
acia_rx_buffer_read_ptr = $02FF
acia_tx_buffer = $0300
acia_rx_buffer = $0400
