    .global _start

_start:
    mov $1, %rax        # sys_write
    mov $1, %rdi        # stdout
    mov $msg, %rsi      # address of string
    mov $len, %rdx      # length
    syscall

    mov $60, %rax       # sys_exit
    xor %rdi, %rdi
    syscall

msg:
    .ascii "Hello, Assembly!\n"
len = . - msg

