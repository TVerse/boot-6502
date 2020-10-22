  .include base.s

DEBUG=1

  .org ROM_START_ADDR

EXPECT_NEXT_ADDR_LOW = $01
EXPECT_NEXT_ADDR_HIGH = $02
EXPECT_NEXT_LEN = $03
EXPECT_NEXT_RECEIVE_DATA = $04
EXPECT_NEXT_SEND_DATA = $05
EXPECT_NEXT_DONE = $FF

  .struct TransferState
done .byte 0
command .byte 0
expect_next .byte 0
length .byte 0
data_pointer .word 0
current_byte_index .byte 0
data_taken_received .byte 0
  .endstruct

  .dsect
  .org $3E00
transfer_state: TransferState
transferred_string: .blk 256
  .dend

COMMAND_DISPLAY_STRING = $FF
COMMAND_WRITE_DATA = $01
COMMAND_READ_DATA = $02

ACK = $01
ACKDATA = $02

handshakes: .byte 0, ACK, ACK, ACKDATA

reset:
  ; Turn on cursor
  JSR wait_lcd_ready
  LDA #%00001110
  JSR lcd_instruction
  STZ INITIALIZATION_DONE

  ;LITERAL waiting
  ;JSR print_null_terminated_string_stack

  STZ transfer_state + TransferState.done
  STZ transfer_state + TransferState.expect_next

  STZ DDRA

  LDA PCR
  AND #%11111000
  ORA #%00001000
  STA PCR
  LDA #%10000010
  STA IER

loop:
  .wait_for_done:
    WAI
    LDA transfer_state + TransferState.done
    BEQ .wait_for_done

  DEBUG_CHAR "P"
  ;AT_ADDRESS_8BIT transfer_state + TransferState.length
  ;AT_ADDRESS transfer_state + TransferState.data_pointer
  ;JSR print_length_string_stack
  .wait_for_handshake:
    WAI
    LDA transfer_state + TransferState.data_taken_received
    BEQ .wait_for_handshake
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
    BNE .ack
    ; TODO also check data_taken_received
    LDA transfer_state + TransferState.expect_next
    BNE .continue_transfer
    .start_transfer:
      JSR start_transfer
      BRA .buttons
    .continue_transfer:
      JSR continue_transfer
      BRA .buttons
    .ack:
      DEBUG_CHAR "A"
      LDA transfer_state + TransferState.data_taken_received
      BNE .buttons ; TODO shouldn't get here?
      .outgoing_handshake:
        LDA #$FF ; TODO 0b00000010 not turning interrupt off?
        STA IFR
        LDA #DEFAULT_DDRA
        STA DDRA
        INC transfer_state + TransferState.data_taken_received
        STZ transfer_state + TransferState.done
        BRA .buttons
  .buttons:
    JSR read_buttons
  .done:
  PLA
  RTI

start_transfer:
  DEBUG_CHAR "S"
  STZ transfer_state + TransferState.done
  STZ transfer_state + TransferState.command
  STZ transfer_state + TransferState.expect_next
  STZ transfer_state + TransferState.length
  STZ transfer_state + TransferState.data_pointer
  STZ transfer_state + TransferState.data_pointer + 1
  STZ transfer_state + TransferState.current_byte_index
  STZ transfer_state + TransferState.data_taken_received
  LDA PORTA
  STA transfer_state + TransferState.command
  CMP #COMMAND_DISPLAY_STRING
  BEQ .display_string
  CMP #COMMAND_WRITE_DATA
  BEQ .write_data
  CMP #COMMAND_READ_DATA
  BEQ .read_data
  BRA .unknown
  .display_string:
    LDA #<transferred_string
    STA transfer_state + TransferState.data_pointer
    LDA #>transferred_string
    STA transfer_state + TransferState.data_pointer + 1
    LDA #EXPECT_NEXT_LEN
    STA transfer_state + TransferState.expect_next
    BRA .return
  ; TODO merge
  .write_data:
    LDA #EXPECT_NEXT_ADDR_LOW
    STA transfer_state + TransferState.expect_next
    BRA .return
  .read_data:
    LDA #EXPECT_NEXT_ADDR_LOW
    STA transfer_state + TransferState.expect_next
    BRA .return
  .unknown:
    LITERAL unknown_command_error
    JMP error
  .return:
    RTS

continue_transfer:
  LDA transfer_state + TransferState.expect_next
  CMP #EXPECT_NEXT_LEN
  BEQ .length
  CMP #EXPECT_NEXT_ADDR_LOW
  BEQ .addr_low
  CMP #EXPECT_NEXT_ADDR_HIGH
  BEQ .addr_high
  CMP #EXPECT_NEXT_RECEIVE_DATA
  BEQ .receive_data
  CMP #EXPECT_NEXT_SEND_DATA
  BEQ .send_data
  .addr_low:
    ;DEBUG_CHAR "A"
    ;DEBUG_CHAR "L"
    LDA PORTA
    STA transfer_state + TransferState.data_pointer
    ;DEBUG_A
    LDA #EXPECT_NEXT_ADDR_HIGH
    STA transfer_state + TransferState.expect_next
    BRA .return
  .addr_high:
    ;DEBUG_CHAR "A"
    ;DEBUG_CHAR "H"
    LDA PORTA
    STA transfer_state + TransferState.data_pointer + 1
    ;DEBUG_A
    LDA transfer_state + TransferState.command
    LDA #EXPECT_NEXT_LEN
    STA transfer_state + TransferState.expect_next
    BRA .return
  .length:
    ;DEBUG_CHAR "L"
    LDA PORTA
    STA transfer_state + TransferState.length
    DEBUG_A
    LDA transfer_state + TransferState.command
    CMP #COMMAND_READ_DATA
    BEQ .send_data
    .next_receive_data:
      LDA #EXPECT_NEXT_RECEIVE_DATA
      STA transfer_state + TransferState.expect_next
      BRA .return
    .next_send_data:
      LDA #EXPECT_NEXT_SEND_DATA
      STA transfer_state + TransferState.expect_next
      BRA .return
  .receive_data:
    ;DEBUG_CHAR "D"
    PHY
    LDY transfer_state + TransferState.current_byte_index
    LDA transfer_state + TransferState.data_pointer
    STA N_IRQ
    LDA transfer_state + TransferState.data_pointer + 1
    STA N_IRQ + 1
    LDA PORTA
    STA (N_IRQ), Y
    PLY
    INC transfer_state + TransferState.current_byte_index
    LDA transfer_state + TransferState.length
    CMP transfer_state + TransferState.current_byte_index
    BNE .return
    BRA .done
  .send_data:
    ; TODO
    BRA .done
  .done:
    DEBUG_CHAR "X"
    LDA #$FF
    STA DDRA
    PHX
    LDX transfer_state + TransferState.command
    LDA handshakes, X
    PLX
    STA PORTA
    INC transfer_state + TransferState.done
    STZ transfer_state + TransferState.expect_next
  .return:
    RTS


unknown_command_error: .asciiz "Unknown command!"

  .org PROGRAM_NMI_VECTOR
  .word nmi
  .org PROGRAM_RESET_VECTOR
  .word reset
  .org PROGRAM_IRQ_VECTOR
  .word irq
