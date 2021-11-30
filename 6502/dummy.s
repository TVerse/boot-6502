  .include base.s

  .org ROM_START_ADDR

DEBUG=1

reset:
  STZ initialization_done

  LDA #<nmi
  STA program_nmi
  LDA #>nmi
  STA program_nmi + 1

  LDA #%01111111
  STA VIA_IER

  ; Timer
  ; 1250 cycles -> 1248 -> 0x04E0
  ; OR HALF THIS? (625 -> 623 -> 0x026F)
  ; Continuous mode, PB7 square wave
  LDA #%11000000
  STA VIA_ACR
  LDA #$6F
  STA VIA_T1CL
  LDA #$02
  STA VIA_T1CH

  ; ACIA
  ; 1 stop bit, 8 bits, rcv baud rate, 16x
  LDA #%00010000
  STA ACIA_CONTROL_REGISTER
  ; No parity, no echo, interrupt, ready
  LDA #%11000101
  STA ACIA_COMMAND_REGISTER

loop:
  WAI
  JMP loop

hello_world:
  .asciiz "Hi!"

nmi:
  PHA
  LDA ACIA_STATUS_RESET_REGISTERS
  AND #%00001000
  BEQ .done
  LDA ACIA_DATA_REGISTERS
  STA ACIA_DATA_REGISTERS
  JSR print_char
.done:
  PLA
  RTI
irq:
  RTI

