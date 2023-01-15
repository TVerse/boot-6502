    .export SOFTWARE_STACK_START
    .export N
    .export INITIALIZATION_DONE
    .export TEN_MS_COUNTER_ADDR


.zeropage
SOFTWARE_STACK_START = $F5 ; Grows down

N = SOFTWARE_STACK_START + 1 ; 8 bytes

.data
INITIALIZATION_DONE: .byte 0
TEN_MS_COUNTER_ADDR: .word 0
