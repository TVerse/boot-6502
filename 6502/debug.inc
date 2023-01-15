DEBUG=1
.import print_char
.import byte_in_a_to_hex

.macro DEBUG_CHAR char
.ifdef DEBUG
    pha
    lda #char
    jsr print_char
    pla
.endif
.endmacro

.macro DEBUG_A
.ifdef DEBUG
    php
    jsr byte_in_a_to_hex
    plp
.endif
.endmacro
