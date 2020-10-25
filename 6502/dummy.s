  .include base.s

;DEBUG=1

  .org ROM_START_ADDR

reset:
  STZ initialization_done

loop:
  WAI
  JMP loop

nmi:
irq:
  PHA
  JSR read_buttons
  PLA
  RTI

  .org $E000
  LDA #$FF
  STA $0300
  LITERAL str
  JMP print_null_terminated_string_stack
  
str:
  .asciiz "JSR"

  .org PROGRAM_NMI_VECTOR
  .word nmi
  .org PROGRAM_RESET_VECTOR
  .word reset
  .org PROGRAM_IRQ_VECTOR
  .word irq
