.text
.global _Read_x1
.global _Read_x2
.global _Read_x3
.global _Read_x4


_Read_x1:
loop1:
    ldr x2,[x1]
    subs x0, x0, 1
    bhi loop1
    ret

_Read_x2:
loop2:
    ldr x2,[x1]
    ldr x2,[x1]
    subs x0, x0, 2
    bhi loop2
    ret

_Read_x3:
loop3:
    ldr x2,[x1]
    ldr x2,[x1]
    ldr x2,[x1]
    subs x0, x0, 3
    bhi loop3
    ret

_Read_x4:
loop4:
    ldr x2,[x1]
    ldr x2,[x1]
    ldr x2,[x1]
    ldr x2,[x1]
    subs x0, x0, 4
    bhi loop4
    ret
