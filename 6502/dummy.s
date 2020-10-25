  .include base.s

;DEBUG=1

  .org ROM_START_ADDR
  .include comms.s

reset:
  ; Turn on cursor
  JSR wait_lcd_ready
  LDA #%00001110
  JSR lcd_instruction
  STZ INITIALIZATION_DONE

  JSR set_input
  JSR init

loop:
  WAI
  JMP loop

nmi:
irq:
  PHA
  LDA IFR
  BPL .buttons ; Not the VIA?
  AND #%00000010 ; CA2 (handshake)
  BEQ .buttons
    JSR dispatch
  .buttons:
    JSR read_buttons
  .done:
    PLA
    RTI

  .org PROGRAM_NMI_VECTOR
  .word nmi
  .org PROGRAM_RESET_VECTOR
  .word reset
  .org PROGRAM_IRQ_VECTOR
  .word irq
