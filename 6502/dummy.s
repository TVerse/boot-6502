.include "stack.inc"
.include "via.inc"
.include "debug.inc"
.include "acia.inc"

.import INITIALIZATION_DONE

.export reset

reset:
    stz INITIALIZATION_DONE

loop:
    jsr acia_block_handle_message
    wai
    jmp loop

nmi:
    rti
irq:
    rti

.rodata
hello_world:
.asciiz "Hello, world! How are you?"
