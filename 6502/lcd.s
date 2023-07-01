.include "stack.inc"
.include "via.inc"
.include "zeropage.inc"

.export LCD_CLEAR
.export initialize_lcd
.export print_char
.export lcd_instruction
.export print_null_terminated_string_stack

.bss
LCD_CLEAR = %00001100

E  = %10000000
RW = %00000010
RS = %00000001
DATA = %00111100

.code

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
  ; 4-bit, 2-line, font 0
    lda #%00101000
    jsr lcd_instruction
  ; Display on, cursor off, blink off
    lda #%00001100
    jsr lcd_instruction
  ; Clear display
    lda #%00000001
    jsr lcd_instruction

    literal_8bit 100
    jsr delay

  ; Increment, no shift
    lda #%00000110
    jsr lcd_instruction
    rts

lcd_instruction:
    jsr wait_lcd_ready
    pha
    jsr lcd_send_upper_nibble
    pla
    jsr lcd_send_lower_nibble
    rts

wait_lcd_ready:
    pha
    lda VIA_DDRB
    pha
    lda #(E | RS | RW)
    sta VIA_DDRB
@poll:
    lda #RW
    sta VIA_PORTB
    eor #E
    sta VIA_PORTB
    eor #E
    sta VIA_PORTB
  ; TODO figure out a way to replace this with BIT again
  ; Can't use PB6 anymore due to VIA
    lda VIA_PORTB
    pha
    lda #RW
    sta VIA_PORTB
    eor #E
    sta VIA_PORTB
    pla
    and #%00100000
    cmp #%00100000
    bne @poll
    lda #RW
    sta VIA_PORTB
    pla
    sta VIA_DDRB
    pla
    rts

lcd_send_upper_nibble:
    lsr
    lsr
    and #DATA
    sta VIA_PORTB
    eor #E
    sta VIA_PORTB
    eor #E
    sta VIA_PORTB
    rts

lcd_send_lower_nibble:
    asl
    asl
    and #DATA
    sta VIA_PORTB
    eor #E
    sta VIA_PORTB
    eor #E
    sta VIA_PORTB
    rts

print_char:
    jsr wait_lcd_ready
    pha
    lsr
    lsr
    and #DATA
    eor #RS
    sta VIA_PORTB
    eor #E
    sta VIA_PORTB
    eor #E
    sta VIA_PORTB
    pla
    asl
    asl
    and #DATA
    eor #RS
    sta VIA_PORTB
    eor #E
    sta VIA_PORTB
    eor #E
    sta VIA_PORTB
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
    sta ptr1
    lda 1,X
    sta ptr1
    pop
    lda 0,X
    ldy #0
@loop:
    lda (ptr1),Y
    jsr print_char
    iny
    tya
    cmp 0, X
    bne @loop
    pop
    rts
