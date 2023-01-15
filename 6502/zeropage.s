.export SOFTWARE_STACK_START
.export ptr1, ptr2, ptr3

.zeropage
STACK_TOP: .res $7F
; stack grows down
SOFTWARE_STACK_START: .res 1                              ; Actual size $80

; Scratch space, not required to be persisted
ptr1: .res 2
ptr2: .res 2
ptr3: .res 2
