.include "stack.inc"

.import TEN_MS_COUNTER_ADDR
.import __VIA_START__

.export via_prep_for_transmit
.export delay
.export DEFAULT_DDRA
.export DEFAULT_DDRB
.export init_via
.export VIA_PORTB
.export VIA_PORTA
.export VIA_DDRB
.export VIA_DDRA
.export VIA_T1CL
.export VIA_T1CH
.export VIA_T1LL
.export VIA_T1LH
.export VIA_T2CL
.export VIA_T2CH
.export VIA_SR
.export VIA_ACR
.export VIA_PCR
.export VIA_IFR
.export VIA_IER
.export VIA_PORTA_NOHS

VIA_PORTB  = __VIA_START__ + $00
VIA_PORTA  = __VIA_START__ + $01
VIA_DDRB = __VIA_START__ + $02
VIA_DDRA  = __VIA_START__ + $03
VIA_T1CL = __VIA_START__ + $04
VIA_T1CH  = __VIA_START__ + $05
VIA_T1LL  = __VIA_START__ + $06
VIA_T1LH  = __VIA_START__ + $07
VIA_T2CL  = __VIA_START__ + $08
VIA_T2CH  = __VIA_START__ + $09
VIA_SR  = __VIA_START__ + $0A
VIA_ACR  = __VIA_START__ + $0B
VIA_PCR  = __VIA_START__ + $0C
VIA_IFR  = __VIA_START__ + $0D
VIA_IER  = __VIA_START__ + $0E
VIA_PORTA_NOHS  = __VIA_START__ + $0F

  ; Start 5ms clock, 10000 cycles @ 1MHz
  ; 2 cycles for starting the interrupt = 9998 wait = $270E
TIMER_CYCLES = $270E

.code
; DEFAULT_DDRA = %00000000
DEFAULT_DDRA = %11111111
DEFAULT_DDRB = %10111111

init_via:
 ; Set data direction
    lda #DEFAULT_DDRA
    sta VIA_DDRA
    lda #DEFAULT_DDRB
    sta VIA_DDRB
 ; Put ports in known state
    stz VIA_PORTA
    stz VIA_PORTB

  ; Reset counter
    stz TEN_MS_COUNTER_ADDR
    stz TEN_MS_COUNTER_ADDR + 1

  ; Enable timer
    lda #<TIMER_CYCLES
    sta VIA_T1CL
    lda #>TIMER_CYCLES
    sta VIA_T1CH

    lda VIA_ACR
    and #%01111111
    ora #%01000000
    sta VIA_ACR

    lda #%11000000
    sta VIA_IER
    rts

; ( 10ms_cycle_count -- )
; Clobbers A
delay:
    clc
    lda TEN_MS_COUNTER_ADDR
    adc 0, X
    sta 0, X
    lda TEN_MS_COUNTER_ADDR + 1
    adc 1, X
    sta 1, X
@loop:
    wai
    lda 0, X
    cmp TEN_MS_COUNTER_ADDR
    bne @loop
    lda 1, X
    cmp TEN_MS_COUNTER_ADDR + 1
    bne @loop
    pop
    rts

via_prep_for_transmit:
  ; Set T2 to pulse counting mode
    lda VIA_ACR
    eor #%00100000
    sta VIA_ACR
    lda #%10100000
    sta VIA_IER
    rts
