ROM_START_ADDR = $8000
VECTORS_START_ADDR = $FFFA

PORTB = $6000
PORTA = $6001
DDRB = $6002
DDRA = $6003
T1CL = $6004
T1CH = $6005
T1LL = $6006
T1LH = $6007
T2CL = $6008
T2CH = $6009
SR = $600A
ACR = $600B
PCR = $600C
IFR = $600D
IER = $600E
PORTA_NOHS = $600F

E  = %00000100
RW = %00000010
RS = %00000001

DEFAULT_DDRA = %00000000
DEFAULT_DDRB = %01111111

  .struct TransferState
done .byte 0
command .byte 0
next .byte 0
length .byte 0
jmp .byte 0
data_pointer .word 0
current_byte_index .byte 0
data_taken_received .byte 0
  .endstruct

  .dsect
  .org $3EFA
program_nmi: .word 0
  .org $3EFC
program_reset: .word 0
  .org $3EFE
program_irq: .word 0

  .org $3F00
transfer_state: TransferState
transferred_string: .blk 128

  .org $3FF0
five_millisecond_counter_addr: .word 0
; 8 bit, positive or zero if done, negative if not. To be set to zero by user. Used in base irq routine, functions as user-only cli.
initialization_done: .byte 0
button_state_addr: .byte 0
  .org $3FFF
program_load_done: .byte 0
  .dend


SOFTWARE_STACK_START = $F5

N = SOFTWARE_STACK_START + 1 ; 8 bytes
N_IRQ = N + 8; 2 bytes
