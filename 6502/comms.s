EXPECT_NEXT_COMMAND = $00
EXPECT_NEXT_ADDR_LOW = $01
EXPECT_NEXT_ADDR_HIGH = $02
EXPECT_NEXT_LEN = $03
EXPECT_NEXT_DATA = $04
SEND_NEXT_DATA = $05
DATA_SENT = $06
ACK_SENT = $FF

  .struct TransferState
done .byte 0
command .byte 0
next .byte 0
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

; IDEA: encode sequence of operation for easier branching
COMMAND_DISPLAY_STRING = $00
COMMAND_WRITE_DATA = $01
COMMAND_READ_DATA = $02

ACK = $01
ACKDATA = $02

handshakes: .byte ACK, ACK, ACKDATA, ACK, ACK

unknown_command_error: .asciiz "Unknown command!"

dispatch:
  LDA transfer_state + TransferState.next
  BEQ .next_receive_command
  CMP #EXPECT_NEXT_ADDR_LOW
  BEQ .next_addr_low
  CMP #EXPECT_NEXT_ADDR_HIGH
  BEQ .next_addr_high
  CMP #EXPECT_NEXT_LEN
  BEQ .next_len
  CMP #EXPECT_NEXT_DATA
  BEQ .next_get_data
  CMP #ACK_SENT
  BEQ .next_after_ack
  CMP #SEND_NEXT_DATA
  BEQ .next_send_data
  CMP #DATA_SENT
  BEQ .next_data_sent
    DEBUG_CHAR "E"
  .error:
    JMP .error
  .next_receive_command:
    JMP receive_command
  .next_addr_low:
    JMP receive_addr_low
  .next_addr_high:
    JMP receive_addr_high
  .next_len:
    JMP receive_len
  .next_get_data:
    JMP receive_data
  .next_after_ack:
    JMP after_ack
  .next_send_data:
    JMP send_data
  .next_data_sent:
    JMP done

set_input:
  LDA #$FF ; Why does #%00000010 not work?
  STA IFR ; TODO why do I need to turn interrupt off manually? Going to STA PORTA later
  STZ DDRA
  RTS

init:
  LDA PCR
  AND #%11111001
  ORA #%00001000
  STA PCR
  LDA #%10000010
  STA IER
  STZ transfer_state + TransferState.command
  STZ transfer_state + TransferState.done
  STZ transfer_state + TransferState.next
  STZ transfer_state + TransferState.current_byte_index
  RTS

receive_command:
  LDA PORTA
  STA transfer_state + TransferState.command
  BEQ .display_string
  CMP #COMMAND_WRITE_DATA
  BEQ .write_data
  CMP #COMMAND_READ_DATA
  BEQ .read_data
  .unknown:
    LITERAL unknown_command_error
    JMP error
  .display_string:
    LDA #<transferred_string
    STA transfer_state + TransferState.data_pointer
    LDA #>transferred_string
    STA transfer_state + TransferState.data_pointer + 1
    LDA #EXPECT_NEXT_LEN
    STA transfer_state + TransferState.next
    RTS
  .write_data:
  .read_data:
    LDA #EXPECT_NEXT_ADDR_LOW
    STA transfer_state + TransferState.next
    RTS

receive_addr_low:
  LDA PORTA
  STA transfer_state + TransferState.data_pointer
  LDA #EXPECT_NEXT_ADDR_HIGH
  STA transfer_state + TransferState.next
  RTS

receive_addr_high:
  LDA PORTA
  STA transfer_state + TransferState.data_pointer + 1
  LDA #EXPECT_NEXT_LEN
  STA transfer_state + TransferState.next
  RTS

receive_len:
  DEBUG_CHAR "L"
  LDA PORTA
  DEBUG_A
  STA transfer_state + TransferState.length
  LDA transfer_state + TransferState.command
  CMP #COMMAND_READ_DATA
  BEQ .next_ack
  .next_receive_data:
    LDA #EXPECT_NEXT_DATA
    STA transfer_state + TransferState.next
    RTS
  .next_ack:
    JMP send_ack

receive_data:
  DEBUG_CHAR "D"
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
  BEQ .next_ack
  .return:
    RTS
  .next_ack:
    LDA transfer_state + TransferState.command
    BNE .ack
      AT_ADDRESS_8BIT transfer_state + TransferState.length
      AT_ADDRESS transfer_state + TransferState.data_pointer
      JSR print_length_string_stack
    .ack:
      JMP send_ack

set_output:
  LDA #$FF
  STA DDRA
  RTS

send_ack:
  JSR set_output
  PHX
  LDX transfer_state + TransferState.command
  LDA handshakes, X
  PLX
  STA PORTA
  LDA #ACK_SENT
  STA transfer_state + TransferState.next
  RTS

send_data:
  PHY
  LDY transfer_state + TransferState.current_byte_index
  LDA transfer_state + TransferState.data_pointer
  STA N_IRQ
  LDA transfer_state + TransferState.data_pointer + 1
  STA N_IRQ + 1
  LDA (N_IRQ), Y
  STA PORTA
  PLY
  INC transfer_state + TransferState.current_byte_index
  LDA transfer_state + TransferState.length
  CMP transfer_state + TransferState.current_byte_index
  BEQ .next_done
  .return:
    RTS
  .next_done:
    LDA #DATA_SENT
    STA transfer_state + TransferState.next
    RTS

after_ack:
  LDA transfer_state + TransferState.command
  CMP #COMMAND_READ_DATA
  BEQ .read_data
    JMP done
  .read_data:
    LDA #SEND_NEXT_DATA
    STA transfer_state + TransferState.next
    RTS

done:
  INC transfer_state + TransferState.done
  STZ transfer_state + TransferState.next
  STZ transfer_state + TransferState.current_byte_index
  JMP set_input
