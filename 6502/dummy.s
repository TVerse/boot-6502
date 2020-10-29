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

