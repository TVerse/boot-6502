  .include base.s

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

