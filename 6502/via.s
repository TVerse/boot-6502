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
  RTS

; ( 5ms_cycle_count -- )
; Clobbers A
delay:
  CLC
  LDA TEN_MS_COUNTER_ADDR
  ADC 0, X
  STA 0, X
  LDA TEN_MS_COUNTER_ADDR + 1
  ADC 1, X
  STA 1, X
  @loop:
    WAI
    LDA 0, X
    CMP TEN_MS_COUNTER_ADDR
    BNE @loop
    LDA 1, X
    CMP TEN_MS_COUNTER_ADDR + 1
    BNE @loop
  POP
  RTS
via_prep_for_transmit:
  ; Set T2 to pulse counting mode
  LDA VIA_ACR
  EOR #%00100000
  STA VIA_ACR
  LDA #%10100000
  STA VIA_IER
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
  LDA #(E | RS | RW)
  STA VIA_DDRB
  @poll:
    LDA #RW
    STA VIA_PORTB
    EOR #E
    STA VIA_PORTB
    BIT VIA_PORTB
    LDA #RW
    STA VIA_PORTB
    EOR #E
    STA VIA_PORTB
    BVS @poll
  LDA #RW
  STA VIA_PORTB
  PLA
  STA VIA_DDRB
  PLA
  RTS

lcd_send_upper_nibble:
  LSR
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
  AND #DATA
  EOR #RS
  STA VIA_PORTB
  EOR #E
  STA VIA_PORTB
  EOR #E
  STA VIA_PORTB
  RTS

print_null_terminated_string_stack:
  @loop:
    LDA (0,X)
    BEQ @end
    JSR print_char
    INC 0,X
    BNE @loop
    INC 1,X
    BRA @loop
  @end:
    RTS

print_length_string_stack:
  LDA 0,X
  STA z:N + 6
  LDA 1,X
  STA z:N + 7
  POP
  LDA 0,X
  LDY #0
  @loop:
    LDA (N + 6),Y
    JSR print_char
    INY
    TYA
    CMP 0, X
    BNE @loop
  POP
  RTS

