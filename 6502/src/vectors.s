.include "irq.inc"
.include "reset.inc"
.include "nmi.inc"

.segment "VECTORS"

.word Nmi::nmi
.word Reset::reset
.word Irq::irq
