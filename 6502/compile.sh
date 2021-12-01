#!/bin/bash

set -x
vasm6502_oldstyle -wdc02 -Fbin -dotdir $1.s -o $1.bin -L $1.lst
