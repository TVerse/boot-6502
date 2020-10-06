  .include base.s

  .org ROM_START_ADDR

TRANSFER_READY = $1000
TRANSFERRED_BYTE = $1001

reset:
  ; Turn on cursor
  JSR wait_lcd_ready
  LDA #%00001110
  JSR lcd_instruction
  STZ INITIALIZATION_DONE

  ;LITERAL waiting
  ;JSR print_string_stack

  STZ TRANSFER_READY

  STZ DDRA

  LDA PCR
  AND #%11111001
  ORA #%00001001
  STA PCR
  LDA #%10000010
  STA IER
  LDA PORTA

loop:
  WAI
  LDA TRANSFER_READY
  BEQ loop
  LDA TRANSFERRED_BYTE
  JSR print_char
  STZ TRANSFER_READY
  ; LDA TRANSFERRED_BYTE
  ; CMP #%10101010
  ; BNE .error
  .continue:
    JMP loop
  .error:
    JSR sr_error

sr_error:
  STZ TRANSFERRED_BYTE + 1
  LITERAL TRANSFERRED_BYTE
  JMP error

waiting:
  .asciiz "Reading buttons"

nmi:
irq:
  PHA
  LDA IFR
  BPL .buttons ; Not the VIA?
  AND #%00000010
  BEQ .buttons
    ; TODO avoid going too fast
    ; Does LDA PORTA really have to happen here?
    INC TRANSFER_READY
    LDA PORTA
    STA TRANSFERRED_BYTE
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
