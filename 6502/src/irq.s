.import "via.inc"

.export irq

.proc irq
    jsr Via::inc_timer
    rti
.endproc