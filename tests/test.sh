#!/bin/#!/bin/bash

way1() {
    perf top -p $(pidof rmg)
}

way2() {
    perf record -g -p $(pidof rmg)
    perf report -g -n
}

way3() {
    perf record --call-graph dwarf -p $(pidof rmg)
    perf report -g graph --no-children
}

#https://www.brendangregg.com/perf.html

#   4.88%  [kernel]                     [k] copy_user_generic_string
#   3.90%  rmg                          [.] 0x00000000002006b0
#   2.83%  libc.so.6                    [.] 0x00000000000a770b
#   2.21%  rmg                          [.] 0x00000000000e6729
#   1.86%  rmg                          [.] 0x00000000000e6710
#   1.69%  libc.so.6                    [.] 0x00000000000a7713
#   1.64%  rmg                          [.] 0x0000000000200207
#   1.58%  libc.so.6                    [.] 0x00000000000a770f
#   1.40%  rmg                          [.] 0x00000000002006bf
#   1.39%  libc.so.6                    [.] 0x00000000000a6f18
#   1.31%  libc.so.6                    [.] 0x00000000000a7707
#   1.19%  libc.so.6                    [.] 0x00000000000a7716
#   1.16%  rmg                          [.] 0x00000000000e6689
#   1.13%  rmg                          [.] 0x00000000001331e6
#   1.03%  rmg                          [.] 0x000000000004b3de
#   0.96%  rmg                          [.] 0x000000000004cbe3
#   0.95%  rmg                          [.] 0x000000000004b312
#   0.94%  rmg                          [.] 0x000000000004b34b
#   0.87%  rmg                          [.] 0x00000000001331d9
#   0.78%  rmg                          [.] 0x00000000000e66a9
#   0.76%  rmg                          [.] 0x00000000000e66ed
#   0.75%  rmg                          [.] 0x000000000004b411
