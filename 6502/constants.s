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

SOFTWARE_STACK_START = $F5 ; Grows down

N = SOFTWARE_STACK_START + 1 ; 8 bytes
N_IRQ = N + 8; 2 bytes

program_nmi = $3FFA
program_reset = $3FFC
program_irq = $3FFE

initialization_done = $0200 ; 1 byte
; 1 byte free
five_millisecond_counter_addr = $0202 ; 2 bytes

