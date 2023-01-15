.include "zeropage.inc"

.export copy_string_from_start

.code
; Stack: start stop -
; Max 256 chars including the null or infinite loop
; Clobbers Y
copy_string_from_start:
  ; Copy to N so we don't have to use X
    lda 0, X
    sta ptr1
    lda 1, X
    sta ptr1 + 1
    lda 2, X
    sta ptr2 + 1
    lda 3, X
    sta ptr2
    ldy #$FF
@loop:
    iny
    lda (ptr1), Y
    sta (ptr2), Y
    bne @loop
    rts
