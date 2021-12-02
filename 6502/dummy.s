  .include base.s

  .org ROM_START_ADDR

DEBUG=1

reset:
  STZ initialization_done
  STZ VIA_PORTA
  PHX
  LDX #0
.send_byte:
  LDA hello_world, X
  BEQ .done
  JSR write_transmit_byte
  INX
  BRA .send_byte
.done
  PLX
  JSR initiate_transmit
  INC VIA_PORTA

  LITERAL hello_world
  JSR print_null_terminated_string_stack

loop:
  WAI
  JMP loop

hello_world:
  .asciiz "Hello, world! How are you?"

nmi:
  RTI
irq:
  RTI

