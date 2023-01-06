    .export SOFTWARE_STACK_START
    .export N
    .export INITIALIZATION_DONE
    .export TEN_MS_COUNTER_ADDR

    .export ACIA_TX_IN_PROGRESS
    .export ACIA_TX_BUFFER_WRITE_PTR
    .export ACIA_TX_BUFFER_READ_PTR
    .export ACIA_RX_BUFFER_WRITE_PTR
    .export ACIA_RX_BUFFER_READ_PTR
    .export ACIA_TX_BUFFER
    .export ACIA_RX_BUFFER

SOFTWARE_STACK_START = $F5 ; Grows down

N = SOFTWARE_STACK_START + 1 ; 8 bytes
N_IRQ = N + 8; 2 bytes

INITIALIZATION_DONE = $0200 ; 1 byte
; 1 byte free
TEN_MS_COUNTER_ADDR = $0202 ; 2 bytes

; One byte with page-aligned buffers. (TODO not needed with indexed addressing?)
; Both pointers are increment-then-use (so pointing at the last byte read/written)
; Buffer is full if (write + 1) == read
; Buffer is empty if write == read
; If these are full pointers instead of an increment, does addressing become simpler?
; Might free up a register. But addition doesn't auto-carry anymore so align to page or do 16-bit add.
ACIA_TX_IN_PROGRESS = $02FB
ACIA_TX_BUFFER_WRITE_PTR = $02FC
ACIA_TX_BUFFER_READ_PTR = $02FD
ACIA_RX_BUFFER_WRITE_PTR = $02FE
ACIA_RX_BUFFER_READ_PTR = $02FF
ACIA_TX_BUFFER = $0300
ACIA_RX_BUFFER = $0400
