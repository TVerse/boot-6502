  .include base.s

  .org ROM_START_ADDR

reset:
  STZ initialization_done

loop:
  WAI
  JMP loop

hello_world:
  .asciiz "Hi!"

nmi:
irq:
  RTI

