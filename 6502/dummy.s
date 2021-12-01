  .include base.s

  .org ROM_START_ADDR

DEBUG=1

reset:
  STZ initialization_done

  LDY #0
.send_byte:
  LDA hello_world, Y
  BEQ .done
  JSR write_transmit_byte
  INY
  BRA .send_byte
.done
  JSR initiate_transmit

loop:
  WAI
  JMP loop

hello_world:
  .asciiz "Hello, world! How are you?"

nmi:
  RTI
irq:
  RTI

