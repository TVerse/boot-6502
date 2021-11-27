; Requires a 5ms timer to be running
  .macro INITIALIZE_LCD
  ; Reset
  LITERAL_8BIT 25
  JSR delay
  LDA #%00110000
  JSR lcd_send_upper_nibble
  LITERAL_8BIT 5
  JSR delay
  LDA #%00110000
  JSR lcd_send_upper_nibble
  LITERAL_8BIT 5
  JSR delay
  LDA #%00110000
  JSR lcd_send_upper_nibble
  LITERAL_8BIT 5
  JSR delay
  ; Set 4bit interface
  LDA #%00100000
  JSR lcd_send_upper_nibble
  LITERAL_8BIT 5
  JSR delay

  ; Software initialize
  LDA #%00101000
  JSR lcd_instruction
  LDA #%00001000
  JSR lcd_instruction
  LDA #%00000001
  JSR lcd_instruction

  LITERAL_8BIT 200
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
  LDA DDRB
  PHA
  LDA #%00000111
  STA DDRB
  .poll:
    LDA #RW
    STA PORTB
    EOR #E
    STA PORTB
    BIT PORTB
    LDA #RW
    STA PORTB
    EOR #E
    STA PORTB
    BVS .poll
  LDA #RW
  STA PORTB
  PLA
  STA DDRB
  PLA
  RTS

lcd_send_upper_nibble:
  LSR
  AND #%01111000
  STA PORTB
  EOR #E
  STA PORTB
  EOR #E
  STA PORTB
  RTS

lcd_send_lower_nibble:
  ASL
  ASL
  ASL
  AND #%01111000
  STA PORTB
  EOR #E
  STA PORTB
  EOR #E
  STA PORTB
  RTS

print_char:
  JSR wait_lcd_ready
  PHA
  LSR
  AND #%01111000
  EOR #RS
  STA PORTB
  EOR #E
  STA PORTB
  EOR #E
  STA PORTB
  PLA
  ASL
  ASL
  ASL
  AND #%01111000
  EOR #RS
  STA PORTB
  EOR #E
  STA PORTB
  EOR #E
  STA PORTB
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

