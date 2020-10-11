  .include base.s

  .org ROM_START_ADDR

  .struct TransferState
in_progress .byte 0
done .byte 0
command .byte 0
has_length .byte 0
length .byte 0
data_pointer .word 0
current_byte_index .byte 0
  .endstruct

  .dsect
  .org $3E00
transfer_state: TransferState
transferred_string: .blk 256
  .dend

COMMAND_DISPLAY_STRING = $FF

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
  AT_ADDRESS_8BIT transfer_state + TransferState.length
  AT_ADDRESS transfer_state + TransferState.data_pointer
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
    BNE .buttons ; TODO what if the other side is too fast? Just get stuck here then...
    LDA transfer_state + TransferState.in_progress
    BNE .continue_transfer
    .start_transfer:
;      LDA #"S"
;      JSR print_char
      STZ transfer_state + TransferState.command
      STZ transfer_state + TransferState.has_length
      STZ transfer_state + TransferState.length
      STZ transfer_state + TransferState.data_pointer
      STZ transfer_state + TransferState.data_pointer + 1
      STZ transfer_state + TransferState.current_byte_index
      INC transfer_state + TransferState.in_progress
      LDA PORTA
      STA transfer_state + TransferState.command
      BRA .buttons
    .continue_transfer:
;      LDA #"C"
;      JSR print_char
      JSR continue_transfer
  .buttons:
    JSR read_buttons
  .done:
  PLA
  RTI

continue_transfer:
  LDA #COMMAND_DISPLAY_STRING
  CMP transfer_state + TransferState.command
  BEQ .display_string
  BRA .unknown
  .display_string:
    LDA transfer_state + TransferState.has_length
    BNE .has_length
    .receive_length:
      LDA #<transferred_string
      STA transfer_state + TransferState.data_pointer
      LDA #>transferred_string
      STA transfer_state + TransferState.data_pointer + 1
      INC transfer_state + TransferState.has_length
      LDA PORTA
      STA transfer_state + TransferState.length
      BRA .return
    .has_length:
      PHY
      LDY transfer_state + TransferState.current_byte_index
      LDA transfer_state + TransferState.data_pointer
      STA N_IRQ
      LDA transfer_state + TransferState.data_pointer + 1
      STA N_IRQ + 1
      LDA PORTA
      STA (N_IRQ),Y
      PLY
      INC transfer_state + TransferState.current_byte_index
      LDA transfer_state + TransferState.length
      CMP transfer_state + TransferState.current_byte_index
      BNE .return
      BRA .done
  .done:
;    LDA #"D"
;    JSR print_char
    INC transfer_state + TransferState.done
    STZ transfer_state + TransferState.in_progress
  .return:
    RTS
  .unknown:
    LITERAL unknown_command_error
    JMP error


unknown_command_error: .asciiz "Unknown command!"

  .org PROGRAM_NMI_VECTOR
  .word nmi
  .org PROGRAM_RESET_VECTOR
  .word reset
  .org PROGRAM_IRQ_VECTOR
  .word irq
