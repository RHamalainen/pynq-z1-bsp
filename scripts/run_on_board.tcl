connect
targets -set -filter {name =~ "ARM*#0"}
rst
fpga pynq/top.bit
loadhw pynq/system.hdf
source pynq/ps7_init.tcl
ps7_init
ps7_post_config
#dow target/armv7a-none-eabi/release/examples/uart_interaction
#con
