.include "via.inc"
.import Via::init

.export reset

.proc reset
    ; Ignore new interrupts
    sei

    ; Reset decimal flag
    cld

    ; Set hw stack pointer
    ldx #$FF
    txs

    ; init VIA
    jsr Via::init

    jsr loop
.endproc

.proc loop
    jmp loop
.endproc
