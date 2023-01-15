.import print_char

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

byte_in_a_to_hex:
    phx
    pha
    tax
    lda #'$'
    jsr print_char
    txa
    lsr
    lsr
    lsr
    lsr
    tax
    lda byte_to_hex_table,X
    jsr print_char
    pla
    pha
    and #%00001111
    tax
    lda byte_to_hex_table,X
    jsr print_char
    pla
    plx
    rts


.rodata
byte_to_hex_table: .literal "0123456789ABCDEF"
