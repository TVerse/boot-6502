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

print_string_stack:
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

; Returns button state in A and BUTTON_STATE_ADDR
; 0000rldu
; Only highest priority bit will be set
; Priority: udlr
read_buttons:
  LDA #DEFAULT_DDRB
  STA DDRB
  .up:
    LDA #%01110000
    STA PORTB
    BIT PORTB
    BMI .down
    LDA #%00000001
    BRA .done
  .down:
    LDA #%01101000
    STA PORTB
    BIT PORTB
    BMI .left
    LDA #%00000010
    BRA .done
  .left:
    LDA #%01011000
    STA PORTB
    BIT PORTB
    BMI .right
    LDA #%00000100
    BRA .done
  .right:
    LDA #%00111000
    STA PORTB
    BIT PORTB
    BMI .nothing
    LDA #%00001000
    BRA .done
  .nothing:
    LDA #0
  .done:
    STA BUTTON_STATE_ADDR
    RTS
