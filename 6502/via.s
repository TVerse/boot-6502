  .include "stack.s"

  .import __VIA_START__

  .importzp N
  .import TEN_MS_COUNTER_ADDR

  .export VIA_PORTB
  .export VIA_PORTA
  .export VIA_DDRB
  .export VIA_DDRA
  .export VIA_T1CL
  .export VIA_T1CH
  .export VIA_T1LL
  .export VIA_T1LH
  .export VIA_T2CL
  .export VIA_T2CH
  .export VIA_SR
  .export VIA_ACR
  .export VIA_PCR
  .export VIA_IFR
  .export VIA_IER
  .export VIA_PORTA_NOHS
  .export via_prep_for_transmit
  .export print_char
  .export delay
  .export DEFAULT_DDRA
  .export DEFAULT_DDRB
  .export LCD_CLEAR
  .export lcd_instruction
  .export print_null_terminated_string_stack
  .export initialize_lcd

VIA_PORTB = __VIA_START__ + $00
VIA_PORTA = __VIA_START__ + $01
VIA_DDRB = __VIA_START__ + $02
VIA_DDRA = __VIA_START__ + $03
VIA_T1CL = __VIA_START__ + $04
VIA_T1CH = __VIA_START__ + $05
VIA_T1LL = __VIA_START__ + $06
VIA_T1LH = __VIA_START__ + $07
VIA_T2CL = __VIA_START__ + $08
VIA_T2CH = __VIA_START__ + $09
VIA_SR = __VIA_START__ + $0A
VIA_ACR = __VIA_START__ + $0B
VIA_PCR = __VIA_START__ + $0C
VIA_IFR = __VIA_START__ + $0D
VIA_IER = __VIA_START__ + $0E
VIA_PORTA_NOHS = __VIA_START__ + $0F

LCD_CLEAR = %00001100

E  = %10000000
RW = %00000010
RS = %00000001
DATA = %00111100

; DEFAULT_DDRA = %00000000
DEFAULT_DDRA = %11111111
DEFAULT_DDRB = %10111111

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

; ( 5ms_cycle_count -- )
; Clobbers A
delay:
  clc
  lda TEN_MS_COUNTER_ADDR
  adc 0, X
  sta 0, X
  lda TEN_MS_COUNTER_ADDR + 1
  adc 1, X
  sta 1, X
  @loop:
    wai
    lda 0, X
    cmp TEN_MS_COUNTER_ADDR
    bne @loop
    lda 1, X
    cmp TEN_MS_COUNTER_ADDR + 1
    bne @loop
  pop
  rts
via_prep_for_transmit:
  ; Set T2 to pulse counting mode
  lda VIA_ACR
  eor #%00100000
  sta VIA_ACR
  lda #%10100000
  sta VIA_IER
  rts

lcd_instruction:
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
    bit VIA_PORTB
    lda #RW
    sta VIA_PORTB
    eor #E
    sta VIA_PORTB
    bvs @poll
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

