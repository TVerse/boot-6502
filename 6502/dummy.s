  .include base.s

  .org ROM_START_ADDR

DEBUG=1

reset:
  STZ initialization_done

  LDA #<nmi
  STA program_nmi
  LDA #>nmi
  STA program_nmi + 1

  ; ACIA
  ; 1 stop bit, 8 bits, rcv baud rate, 9600
  LDA #%00011110
  STA ACIA_CONTROL_REGISTER
  ; No parity, no echo, interrupt, ready
  LDA #%11001001
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

