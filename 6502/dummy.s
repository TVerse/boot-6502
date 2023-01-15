.include "stack.inc"
.include "via.inc"
.include "debug.inc"

;.import write_transmit_byte
;.import blocking_transmit
;.import INITIALIZATION_DONE
;.import copy_string_from_start
;.import initiate_transmit
;.import print_null_terminated_string_stack
;.import Via

.export reset

reset:
;    stz INITIALIZATION_DONE
;    stz VIA_PORTA

; Send 0x55 for ready
    lda #$55
;    jsr write_transmit_byte
;    jsr blocking_transmit
;    inc VIA_PORTA
; Wait until the rx buffer writes a zero at the write pointer
@waiting:
;  ldy IOStatus::RxBufferWriteIdx
;  lda ACIA_RX_BUFFER, Y
    bne @waiting
;    dec VIA_PORTA
@ready:
    literal $3000
;  literal ACIA_TX_BUFFER
  ; TODO does not count as reading!
;    jsr copy_string_from_start
    pop
    phx
    ldx #0
@send_byte:
    lda $3000, X
    php
;    jsr write_transmit_byte
    plp
    beq @done
    inx
    bra @send_byte
@done:
    plx
;    jsr initiate_transmit

;    jsr print_null_terminated_string_stack
    pop

;    inc VIA_PORTA

loop:
    wai
    jmp loop

nmi:
    rti
irq:
    rti

.rodata
hello_world:
.asciiz "Hello, world! How are you?"
