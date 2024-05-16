set $fbase = 0x7100
set $rbase = 0x20001100

add-symbol-file ../../pipcore-mpu/pip.elf

source gdbinit

set architecture arm
target extended-remote :3333
monitor halt
monitor reset
monitor arm semihosting enable
