.include "zeropage.inc"

.import __DATA_LOAD__
.import __DATA_RUN__
.import __DATA_SIZE__

.export copy_string_from_start
.export copy_data

.code

; Clobbers A, X, Y
copy_data:
    ; Source pointer
    lda #<__DATA_LOAD__
    sta ptr1
    lda #>__DATA_LOAD__
    sta ptr1 + 1
    ; Target pointer
    lda #<__DATA_RUN__
    sta ptr2 + 1
    lda #>__DATA_RUN__
    sta ptr2 + 1

    ldx #<~__DATA_SIZE__
    lda #>~__DATA_SIZE__
    sta tmp1
    ldy #$00

@bump_low_counter:
    inx
    beq @bump_high_counter

@copy_loop:
    lda (ptr1), y
    sta (ptr2), y
    iny
    bne @bump_low_counter
    inc ptr1+1
    inc ptr2+1
    bra @bump_low_counter

@bump_high_counter:
    inc tmp1
    bne @copy_loop

    rts

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
