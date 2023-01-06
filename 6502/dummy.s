  .include "macros.s"

  .import VIA_PORTA
  .import write_transmit_byte
  .import blocking_transmit
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
  stz INITIALIZATION_DONE
  stz VIA_PORTA

; Send 0x55 for ready
  lda #$55
  jsr write_transmit_byte
  jsr blocking_transmit
; Wait until the rx buffer writes a zero at the write pointer
  inc VIA_PORTA
@waiting:
  ldy ACIA_RX_BUFFER_WRITE_PTR
  lda ACIA_RX_BUFFER, Y
  bne @waiting
  dec VIA_PORTA
@ready:
  literal $3000
  literal ACIA_TX_BUFFER
  ; TODO does not count as reading!
  jsr copy_string_from_start
  pop
  phx
  ldx #0
@send_byte:
  lda $3000, X
  php
  jsr write_transmit_byte
  plp
  beq @done
  inx
  bra @send_byte
@done:
  plx
  jsr initiate_transmit

  jsr print_null_terminated_string_stack
  pop

;  inc VIA_PORTA

loop:
  wai
  jmp loop

hello_world:
  .asciiz "Hello, world! How are you?"

nmi:
  rti
irq:
  rti

