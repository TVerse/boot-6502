LCD_CLEAR = %00001100

E  = %00000100
RW = %00000010
RS = %00000001
DATA = %01111000

DEFAULT_DDRA = %00000000
DEFAULT_DDRB = %01111111

; Requires a 10ms timer to be running
  .macro INITIALIZE_LCD
  ; Reset
  LITERAL_8BIT 13
  JSR delay
  LDA #%00110000
  JSR lcd_send_upper_nibble
  LITERAL_8BIT 3
  JSR delay
  LDA #%00110000
  JSR lcd_send_upper_nibble
  LITERAL_8BIT 3
  JSR delay
  LDA #%00110000
  JSR lcd_send_upper_nibble
  LITERAL_8BIT 3
  JSR delay
  ; Set 4bit interface
  LDA #%00100000
  JSR lcd_send_upper_nibble
  LITERAL_8BIT 3
  JSR delay

  ; Software initialize
  LDA #%00101000
  JSR lcd_instruction
  LDA #%00001000
  JSR lcd_instruction
  LDA #%00000001
  JSR lcd_instruction

  LITERAL_8BIT 100
  JSR delay

  LDA #%00000110
  JSR lcd_instruction
  .endmacro

lcd_instruction:
  PHA
  JSR lcd_send_upper_nibble
  PLA
  JSR lcd_send_lower_nibble
  RTS

wait_lcd_ready:
  PHA
  LDA VIA_DDRB
  PHA
  LDA #%00000111
  STA VIA_DDRB
  .poll:
    LDA #RW
    STA VIA_PORTB
    EOR #E
    STA VIA_PORTB
    BIT VIA_PORTB
    LDA #RW
    STA VIA_PORTB
    EOR #E
    STA VIA_PORTB
    BVS .poll
  LDA #RW
  STA VIA_PORTB
  PLA
  STA VIA_DDRB
  PLA
  RTS

lcd_send_upper_nibble:
  LSR
  AND #DATA
  STA VIA_PORTB
  EOR #E
  STA VIA_PORTB
  EOR #E
  STA VIA_PORTB
  RTS

lcd_send_lower_nibble:
  ASL
  ASL
  ASL
  AND #DATA
  STA VIA_PORTB
  EOR #E
  STA VIA_PORTB
  EOR #E
  STA VIA_PORTB
  RTS

print_char:
  JSR wait_lcd_ready
  PHA
  LSR
  AND #DATA
  EOR #RS
  STA VIA_PORTB
  EOR #E
  STA VIA_PORTB
  EOR #E
  STA VIA_PORTB
  PLA
  ASL
  ASL
  ASL
  AND #DATA
  EOR #RS
  STA VIA_PORTB
  EOR #E
  STA VIA_PORTB
  EOR #E
  STA VIA_PORTB
  RTS

print_null_terminated_string_stack:
  .loop:
    LDA (0,X)
    BEQ .end
    JSR print_char
    INC 0,X
    BNE .loop
    INC 1,X
    BRA .loop
  .end:
    POP
    RTS

print_length_string_stack:
  LDA 0,X
  STA N + 6
  LDA 1,X
  STA N + 7
  POP
  LDA 0,X
  LDY #0
  .loop:
    LDA (N + 6),Y
    JSR print_char
    INY
    TYA
    CMP 0, X
    BNE .loop
  POP
  RTS

