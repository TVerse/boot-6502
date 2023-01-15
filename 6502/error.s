.include "lcd.inc"
.include "stack.inc"

.export error

; ( string_pointer -- )
; Does not return
error:
    sei
    lda #%00000001
    jsr lcd_instruction
    literal error_message
    jsr print_null_terminated_string_stack
    lda 0,X
    ora 1,X
    beq @loop
    lda #%11000000                  ; Jump to second row
    jsr lcd_instruction
    jsr print_null_terminated_string_stack
@loop:
    wai
    jmp @loop

.rodata
error_message: .asciiz "ERROR: "
