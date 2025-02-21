ROM_START_ADDR = $8000
VECTORS_START_ADDR = $FFFA

VIA_BASE = $6000
VIA_PORTB = VIA_BASE + $0
VIA_PORTA = VIA_BASE + $1
VIA_DDRB = VIA_BASE + $2
VIA_DDRA = VIA_BASE + $3
VIA_T1CL = VIA_BASE + $4
VIA_T1CH = VIA_BASE + $5
VIA_T1LL = VIA_BASE + $6
VIA_T1LH = VIA_BASE + $7
VIA_T2CL = VIA_BASE + $8
VIA_T2CH = VIA_BASE + $9
VIA_SR = VIA_BASE + $A
VIA_ACR = VIA_BASE + $B
VIA_PCR = VIA_BASE + $C
VIA_IFR = VIA_BASE + $D
VIA_IER = VIA_BASE + $E
VIA_PORTA_NOHS = VIA_BASE + $F

ACIA_BASE = $5000
ACIA_DATA_REGISTERS = ACIA_BASE + $0
ACIA_STATUS_RESET_REGISTERS = ACIA_BASE + $1
ACIA_COMMAND_REGISTER = ACIA_BASE + $2
ACIA_CONTROL_REGISTER = ACIA_BASE + $3

SOFTWARE_STACK_START = $F5 ; Grows down

N = SOFTWARE_STACK_START + 1 ; 8 bytes
N_IRQ = N + 8; 2 bytes

program_nmi = $3FFA
program_reset = $3FFC
program_irq = $3FFE

initialization_done = $0200 ; 1 byte
; 1 byte free
ten_millisecond_counter_addr = $0202 ; 2 bytes

; Both pointers are use first, increment second.
; One byte with page-aligned buffers.
acia_tx_buffer_write_ptr = $02FC
acia_tx_buffer_read_ptr = $02FD
acia_rx_buffer_write_ptr = $02FE
acia_rc_buffer_read_ptr = $02FF
acia_tx_buffer = $0300
acia_rx_buffer = $0400
