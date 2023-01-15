.include "stack.inc"
.include "via.inc"

.importzp N
.import delay
.import Via

.export LCD_CLEAR
.export initialize_lcd
.export print_char
.export lcd_instruction
.export print_null_terminated_string_stack

LCD_CLEAR = %00001100

E  = %10000000
RW = %00000010
    RS = %00000001
DATA = %00111100

; Requires a 10ms timer to be running
initialize_lcd:
  ; Reset
    literal_8bit 13
    jsr delay
    lda #%00110000
    jsr lcd_send_upper_nibble
    literal_8bit 3
    jsr delay
    lda #%00110000
    jsr lcd_send_upper_nibble
    literal_8bit 3
    jsr delay
    lda #%00110000
    jsr lcd_send_upper_nibble
    literal_8bit 3
    jsr delay
  ; Set 4bit interface
    lda #%00100000
    jsr lcd_send_upper_nibble
    literal_8bit 3
    jsr delay

  ; Software initialize
    lda #%00101000
    jsr lcd_instruction
    lda #%00001000
    jsr lcd_instruction
    lda #%00000001
    jsr lcd_instruction

    literal_8bit 100
    jsr delay

    lda #%00000110
    jsr lcd_instruction
    rts

lcd_instruction:
    pha
    jsr lcd_send_upper_nibble
    pla
    jsr lcd_send_lower_nibble
    rts

wait_lcd_ready:
    pha
    lda Via+Via::DDRB
    pha
    lda #(E | RS | RW)
    sta Via+Via::DDRB
@poll:
    lda #RW
    sta Via+Via::PortB
    eor #E
    sta Via+Via::PortB
    bit Via+Via::PortB
    lda #RW
    sta Via+Via::PortB
    eor #E
    sta Via+Via::PortB
    bvs @poll
    lda #RW
    sta Via+Via::PortB
    pla
    sta Via+Via::PortB
    pla
    rts

lcd_send_upper_nibble:
    lsr
    lsr
    and #DATA
    sta Via+Via::PortB
    eor #E
    sta Via+Via::PortB
    eor #E
    sta Via+Via::PortB
    rts

lcd_send_lower_nibble:
    asl
    asl
    and #DATA
    sta Via+Via::PortB
    eor #E
    sta Via+Via::PortB
    eor #E
    sta Via+Via::PortB
    rts

print_char:
    jsr wait_lcd_ready
    pha
    lsr
    lsr
    and #DATA
    eor #RS
    sta Via+Via::PortB
    eor #E
    sta Via+Via::PortB
    eor #E
    sta Via+Via::PortB
    pla
    asl
    asl
    and #DATA
    eor #RS
    sta Via+Via::PortB
    eor #E
    sta Via+Via::PortB
    eor #E
    sta Via+Via::PortB
    rts

print_null_terminated_string_stack:
@loop:
    lda (0,X)
    beq @end
    jsr print_char
    inc 0,X
    bne @loop
    inc 1,X
    bra @loop
@end:
    rts

print_length_string_stack:
    lda 0,X
    sta z:N + 6
    lda 1,X
    sta z:N + 7
    pop
    lda 0,X
    ldy #0
@loop:
    lda (N + 6),Y
    jsr print_char
    iny
    tya
    cmp 0, X
    bne @loop
    pop
    rts

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
  ; .has_message:
    lda #%11000000                  ; Jump to second row
    jsr lcd_instruction
    jsr print_null_terminated_string_stack
@loop:
    wai
    jmp @loop

.rodata
error_message: .asciiz "ERROR: "
