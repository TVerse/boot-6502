  .include base.s

  .org ROM_START_ADDR

TRANSFER_IN_PROGRESS = $1000
TRANSFER_DONE = $1001
TRANSFER_LENGTH = $1002
TRANSFER_POINTER = $1003
TRANSFER_RESULT = $1100

reset:
  ; Turn on cursor
  JSR wait_lcd_ready
  LDA #%00001110
  JSR lcd_instruction
  STZ INITIALIZATION_DONE

  ;LITERAL waiting
  ;JSR print_null_terminated_string_stack

  STZ TRANSFER_DONE
  STZ TRANSFER_IN_PROGRESS

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
  LDA TRANSFER_DONE
  BEQ loop
  AT_ADDRESS_8BIT TRANSFER_LENGTH
  LITERAL TRANSFER_RESULT
  JSR print_length_string_stack
  STZ TRANSFER_DONE
  JMP loop

waiting:
  .asciiz "Reading buttons"

nmi:
irq:
  PHA
  LDA IFR
  BPL .buttons ; Not the VIA?
  AND #%00000010
  BEQ .buttons
    LDA TRANSFER_DONE
    BNE .buttons ; TODO what if the other side is too fast?
    LDA TRANSFER_IN_PROGRESS
    BNE .continue_transfer
    .start_transfer:
;      LDA #"S"
;      JSR print_char
      INC TRANSFER_IN_PROGRESS
      LDA PORTA
      STA TRANSFER_LENGTH
      STZ TRANSFER_POINTER
      BRA .buttons
    .continue_transfer:
;      LDA #"C"
;      JSR print_char
      PHY
      LDY TRANSFER_POINTER
      LDA PORTA
      STA TRANSFER_RESULT,Y
      PLY
      INC TRANSFER_POINTER
      LDA TRANSFER_LENGTH
      INC
      CMP TRANSFER_POINTER
      BNE .buttons
;      LDA #"D"
;      JSR print_char
      INC TRANSFER_DONE
      STZ TRANSFER_IN_PROGRESS
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
