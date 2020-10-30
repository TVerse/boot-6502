PORTB = $6000
DDRB = $6002
E  = %00000100
RW = %00000010
RS = %00000001

  .org $0300
  .byte 0
reset:
  LDA #$AA
  STA $0300
  RTS

nmi:
irq:
  RTI
