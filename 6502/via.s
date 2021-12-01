  .ifndef VIA_BASE
VIA_BASE = $4000
  .endif

VIA_PORTB = VIA_BASE + $0
VIA_PORTA = VIA_BASE + $1
VIA_DDRB = VIA_BASE + $2
VIA_DDRA = VIA_BASE + $3
VIA_T1CL = VIA_BASE + $4
VIA_T1CH = VIA_BASE + $5
VIA_T1LL = VIA_BASE + $6
VIA_T1LH = VIA_BASE + $7
VIA_T2CL = VIA_BASE + $8
VIA_T2CH = VIA_BASE + $9
VIA_SR = VIA_BASE + $A
VIA_ACR = VIA_BASE + $B
VIA_PCR = VIA_BASE + $C
VIA_IFR = VIA_BASE + $D
VIA_IER = VIA_BASE + $E
VIA_PORTA_NOHS = VIA_BASE + $F

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

via_prep_for_transmit:
  ; Set T2CL
  STA VIA_T2CL
  ; Set T2 to pulse counting mode
  LDA VIA_ACR
  EOR #%00100000
  STA VIA_ACR
  RTS

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

