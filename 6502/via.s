  .include "stack.inc"
  .include "via.inc"

  .import TEN_MS_COUNTER_ADDR

  .export via_prep_for_transmit
  .export delay
  .export DEFAULT_DDRA
  .export DEFAULT_DDRB
  .export init_via
  .export Via

.segment "VIA"
Via: .tag Via

.code
; DEFAULT_DDRA = %00000000
DEFAULT_DDRA = %11111111
DEFAULT_DDRB = %10111111

init_via:
 ; Set data direction
  lda #DEFAULT_DDRA
  sta Via+Via::DDRA
  lda #DEFAULT_DDRB
  sta Via+Via::DDRB
 ; Put ports in known state
  stz Via+Via::PortA
  stz Via+Via::PortB

  ; Reset counter
  stz TEN_MS_COUNTER_ADDR
  stz TEN_MS_COUNTER_ADDR + 1

  ; Enable timer
  ; Start 5ms clock, 5000 cycles @ 1MHz
  ; 2 cycles for starting the interrupt = 4998 wait = $1368
  lda #$0E
  sta Via+Via::T1CL
  lda #$27
  sta Via+Via::T1CH

  lda Via+Via::ACR
  and #%01111111
  ora #%01000000
  sta Via+Via::ACR

  lda #%11000000
  sta Via+Via::IER
  rts

; ( 5ms_cycle_count -- )
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
  lda Via+Via::ACR
  eor #%00100000
  sta Via+Via::ACR
  lda #%10100000
  sta Via+Via::IER
  rts
