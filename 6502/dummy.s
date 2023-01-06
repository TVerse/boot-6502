  .include "stack.s"

  .import VIA_PORTA
  .import write_transmit_byte
  .import block_transmit
  .import INITIALIZATION_DONE
  .import ACIA_RX_BUFFER_WRITE_PTR
  .import ACIA_RX_BUFFER
  .import ACIA_TX_BUFFER
  .import copy_string_from_start
  .import initiate_transmit
  .import print_null_terminated_string_stack

  .export reset

DEBUG=1

reset:
  STZ INITIALIZATION_DONE
  STZ VIA_PORTA

; Send 0x55 for ready
  LDA #$55
  JSR write_transmit_byte
  JSR block_transmit
; Wait until the rx buffer writes a zero at the write pointer
  INC VIA_PORTA
@waiting:
  LDY ACIA_RX_BUFFER_WRITE_PTR
  LDA ACIA_RX_BUFFER, Y
  BNE @waiting
  DEC VIA_PORTA
@ready:
  LITERAL $3000
  LITERAL ACIA_TX_BUFFER
  ; TODO does not count as reading!
  JSR copy_string_from_start
  POP
  PHX
  LDX #0
@send_byte:
  LDA $3000, X
  PHP
  JSR write_transmit_byte
  PLP
  BEQ @done
  INX
  BRA @send_byte
@done:
  PLX
  JSR initiate_transmit

  JSR print_null_terminated_string_stack
  POP

;  INC VIA_PORTA
loop:
  WAI
  JMP loop

hello_world:
  .asciiz "Hello, world! How are you?"

nmi:
  RTI
irq:
  RTI

