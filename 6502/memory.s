.importzp N

.export copy_string_from_start

.code
; Stack: start stop -
; Max 256 chars including the null or infinite loop
; Clobbers Y
copy_string_from_start:
  ; Copy to N so we don't have to use X
    lda 0, X
    sta N
    lda 1, X
    sta N + 1
    lda 2, X
    sta N + 2
    lda 3, X
    sta N + 3
    ldy #$FF
@loop:
    iny
    lda (N), Y
    sta (N + 2), Y
    bne @loop
    rts
