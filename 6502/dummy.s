  .include base.s

  .org ROM_START_ADDR

reset:
  STZ INITIALIZATION_DONE
  LITERAL waiting
  JSR print_string_stack
loop:
  WAI
  JMP loop

waiting:
  .asciiz "Reading buttons"

nmi:
irq:
  JSR read_buttons
  ASL
  STA PORTA
  RTI

  .org PROGRAM_NMI_VECTOR
  .word nmi
  .org PROGRAM_RESET_VECTOR
  .word reset
  .org PROGRAM_IRQ_VECTOR
  .word irq
