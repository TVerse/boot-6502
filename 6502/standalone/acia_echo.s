ROM_START_ADDR = $8000
VECTORS_START_ADDR = $FFFA

VIA_BASE = $6000
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

ACIA_BASE = $5000
ACIA_DATA_REGISTERS = ACIA_BASE + $0
ACIA_STATUS_RESET_REGISTERS = ACIA_BASE + $1
ACIA_COMMAND_REGISTER = ACIA_BASE + $2
ACIA_CONTROL_REGISTER = ACIA_BASE + $3

  .org ROM_START_ADDR

; Set up timer to 1250 cycles (50 baud)
; Configure ACIA to divisor 16, echo, interrupts off

reset:
  ; Basic stuff
  CLD
  LDX #$FF
  TXS
  STZ VIA_DDRA
  LDA #%10000000
  STA VIA_DDRB

  ; Timer
  ; 1250 cycles -> 1248 -> 0x04E0
  ; OR HALF THIS? (750 -> 748 -> 0x02EC)
  ; Continuous mode, PB7 square wave
  LDA #%11000000
  STA VIA_ACR
  LDA #$EC
  STA VIA_T1CL
  LDA #$02
  STA VIA_T1CH

  ; ACIA
  ; 1 stop bit, 8 bits, rcv baud rate, 16x
  LDA #%0001000
  STA ACIA_CONTROL_REGISTER
  ; No parity, echo, no interrupt, ready
  LDA #%11010011
  STA ACIA_COMMAND_REGISTER


loop:
  WAI
  JMP loop


nmi:
  RTI
irq:
  RTI

  .org VECTORS_START_ADDR
  .word nmi
  .word reset
  .word irq
