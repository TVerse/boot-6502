ROM_START_ADDR = $8000
STATICS_START_ADDR = $E000
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

DEFAULT_DDRA = %11111111
DEFAULT_DDRB = %01111111

PROGRAM_NMI_VECTOR = $FFF0
PROGRAM_RESET_VECTOR = $FFF2
PROGRAM_IRQ_VECTOR = $FFF4

FIVE_MILLISECOND_COUNTER_ADDR = $3FFE ; 16 bit

; 8 bit, zero if done, FF if not. To be set to zero by user. Used in base irq routine, functions as user-only cli.
INITIALIZATION_DONE = $3FF0 
BUTTON_STATE_ADDR = $3FFB ; 0000udlr

SOFTWARE_STACK_START = $F7

N = $F8 ; 8 bytes
