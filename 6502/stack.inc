.macro inx2
    inx
    inx
.endmacro

.macro dex2
    dex
    dex
.endmacro

.macro pop
    inx2
.endmacro

.macro zero
    dex2
    stz 0,X
    stz 1,X
.endmacro

.macro literal lit
    dex2
    lda #<lit
    sta 0,X
    lda #>lit
    sta 1,X
.endmacro

.macro literal_8bit lit
    dex2
    lda #lit
    sta 0,X
    stz 1,X
.endmacro

.macro at_address addr
    dex2
    lda addr
    sta 0,X
    lda addr + 1
    sta 1,X
.endmacro

.macro at_address_8bit addr
    dex2
    lda addr
    sta 0,X
    stz 1,X
.endmacro
