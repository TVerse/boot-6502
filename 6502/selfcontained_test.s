PORTB = $6000
DDRB = $6002

  .org $0300
reset:
  LDA #"H"
  JSR print_char
  LDA #"i"
  JSR print_char
  LDA #"!"
  JSR print_char
loop:
  WAI
  JMP loop

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

nmi:
irq:
  RTI
  .org #3EFA
  .word nmi
  .word reset
  .word irq