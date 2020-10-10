  .include base.s

  .org ROM_START_ADDR

  .struct TransferState
in_progress .byte 0
done .byte 0
current_byte_index .byte 0
result_pointer .word 0
result_length .byte 0
  .endstruct

  .dsect
  .org $0200
transfer_state: TransferState
transfer_result: .blk 256
  .dend

reset:
  ; Turn on cursor
  JSR wait_lcd_ready
  LDA #%00001110
  JSR lcd_instruction
  STZ INITIALIZATION_DONE

  ;LITERAL waiting
  ;JSR print_null_terminated_string_stack

  STZ transfer_state + TransferState.done
  STZ transfer_state + TransferState.in_progress
  LDA #<transfer_result
  STA transfer_state + TransferState.result_pointer
  LDA #>transfer_result
  STA transfer_state + TransferState.result_pointer + 1

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
  LDA transfer_state + TransferState.done
  BEQ loop
  AT_ADDRESS_8BIT transfer_state + TransferState.result_length
  AT_ADDRESS transfer_state + TransferState.result_pointer
  JSR print_length_string_stack
  STZ transfer_state + TransferState.done
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
    LDA transfer_state + TransferState.done
    BNE .buttons ; TODO what if the other side is too fast?
    LDA transfer_state + TransferState.in_progress
    BNE .continue_transfer
    .start_transfer:
;      LDA #"S"
;      JSR print_char
      INC transfer_state + TransferState.in_progress
      LDA PORTA
      STA transfer_state + TransferState.result_length
      STZ transfer_state + TransferState.current_byte_index
      BRA .buttons
    .continue_transfer:
;      LDA #"C"
;      JSR print_char
      PHY
      LDY transfer_state + TransferState.current_byte_index
      LDA transfer_state + TransferState.result_pointer
      STA N_IRQ
      LDA transfer_state + TransferState.result_pointer + 1
      STA N_IRQ + 1
      LDA PORTA
      STA (N_IRQ),Y
      PLY
      INC transfer_state + TransferState.current_byte_index
      LDA transfer_state + TransferState.current_byte_index
      LDA transfer_state + TransferState.result_length
      CMP transfer_state + TransferState.current_byte_index
      BNE .buttons
;      LDA #"D"
;      JSR print_char
      INC transfer_state + TransferState.done
      STZ transfer_state + TransferState.in_progress
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
