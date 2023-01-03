  .include base.s

  .org ROM_START_ADDR

DEBUG=1

reset:
  STZ initialization_done
  STZ VIA_PORTA

; Send 0x55 for ready
  LDA #$55
  JSR write_transmit_byte
  JSR block_transmit
; Wait until the rx buffer writes a zero at the write pointer
  INC VIA_PORTA
.waiting:
  LDY acia_rx_buffer_write_ptr
  LDA acia_rx_buffer, Y
  BNE .waiting
  DEC VIA_PORTA
.ready:
  LITERAL $3000
  LITERAL acia_rx_buffer
  ; TODO does not count as reading!
  JSR copy_string_from_start
  POP
  PHX
  LDX #0
.send_byte:
  LDA $3000, X
  PHP
  JSR write_transmit_byte
  PLP
  BEQ .done
  INX
  BRA .send_byte
.done
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

