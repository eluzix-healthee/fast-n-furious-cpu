.text
.global _TestExample1
.global _TestExample2

; these method receive the following parameters:
; x0: number of iterations
; x1: pointer to the memory

_TestExample1:
loop1:
    adds x2, x2, 1
    adds x2, x2, 1
;    str x0, [x1]
    subs x0, x0, 1
    bhi loop1
    ret

_TestExample2:
loop2:
    mov x2, x0
    adds x2, x2, 1
    mov x2, x0
    adds x2, x2, 1
;    str x0, [x1]
    subs x0, x0, 1
    bhi loop2
    ret
