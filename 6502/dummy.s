  .include base.s

  .org ROM_START_ADDR

reset:
  STZ INITIALIZATION_DONE
  LITERAL waiting
  JSR print_string_stack
  .read_buttons:
    JSR read_buttons
    BEQ .read_buttons
  LDA #%00000001
  JSR lcd_instruction
  .loop:
    JSR read_byte
    JSR print_char
    JMP .loop

nmi:
irq:
  RTI

waiting:
  .asciiz "Press when ready"

  .org PROGRAM_NMI_VECTOR
  .word nmi
  .org PROGRAM_RESET_VECTOR
  .word reset
  .org PROGRAM_IRQ_VECTOR
  .word irq
