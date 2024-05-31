section .boot
header_start:
        dd 0xe85250d6                ; magic number (multiboot 2)
        dd 0                         ; architecture 0 (protected mode i386)
        dd header_end - header_start ; header length
        ; checksum
        dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

        ; insert optional multiboot tags here

        ; required end tag
        dw 0    ; type
        dw 0    ; flags
        dd 8    ; size
header_end:

global start

section .text
bits 32
start:
        ; update stackpointer
        lea esp, [rel stack_top];[stack_top]

        call check_multiboot
        call check_cpuid
        call check_long_mode

        ; print `OK` to screen
        mov dword [0xb8000], 0x2f4b2f4f

        ; get external function rust main
        extern kmain
        call kmain

; multi boot check
check_multiboot:
        cmp eax, 0x36d76289
        jne .no_multiboot
        ret
.no_multiboot:
        mov al, "0"
        jmp error

check_cpuid:
        ; Check if CPUID is supported by attempting to flip the ID bit (bit 21)
        ; in the FLAGS register. If we can flip it, CPUID is available.

        ; Copy FLAGS in to EAX via stack
        pushfd
        pop eax

        ; Copy to ECX as well for comparing later on
        mov ecx, eax

        ; Flip the ID bit
        xor eax, 1 << 21

        ; Copy EAX to FLAGS via the stack
        push eax
        popfd

        ; Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
        pushfd
        pop eax

        ; Restore FLAGS from the old version stored in ECX (i.e. flipping the
        ; ID bit back if it was ever flipped).
        push ecx
        popfd

        ; Compare EAX and ECX. If they are equal then that means the bit
        ; wasn't flipped, and CPUID isn't supported.
        cmp eax, ecx
        je .no_cpuid
        ret

.no_cpuid:
        mov al, "1"
        jmp error

check_long_mode:
        ; test if extended processor info in available
        mov eax, 0x80000000    ; implicit argument for cpuid
        cpuid                  ; get highest supported argument
        cmp eax, 0x80000001    ; it needs to be at least 0x80000001
        jb .no_long_mode       ; if it's less, the CPU is too old for long mode

        ; use extended info to test if long mode is available
        mov eax, 0x80000001    ; argument for extended processor info
        cpuid                  ; returns various feature bits in ecx and edx
        test edx, 1 << 29      ; test if the LM-bit is set in the D-register
        jz .no_long_mode       ; If it's not set, there is no long mode
        ret
.no_long_mode:
        mov al, "2"
        jmp error

; set_up_page_tables:
;         ; map first P4 entry to P3 table
;         lea eax, [p3_table]
;         or eax, 0b11 ; present + writable
;         mov [p4_table], eax

;         ; map first P3 entry to P2 table
;         mov eax, p2_table
;         or eax, 0b11 ; present + writable
;         mov [p3_table], eax

;         ; TODO map each P2 entry to a huge 2MiB page
;         ret

; .map_p2_table:
;         ; map ecx-th P2 entry to a huge page that starts at address 2MiB*ecx
;         mov eax, 0x200000  ; 2MiB
;         mul ecx            ; start address of ecx-th page
;         or eax, 0b10000011 ; present + writable + huge
;         mov [p2_table + ecx * 8], eax ; map ecx-th entry

;         inc ecx            ; increase counter
;         cmp ecx, 512       ; if counter == 512, the whole P2 table is mapped
;         jne .map_p2_table  ; else map the next entry

;         ret

; error handling while not in long mode
error:
        mov dword [0xb8000], 0x4f524f45
        mov dword [0xb8004], 0x4f3a4f52
        mov dword [0xb8008], 0x4f204f20
        mov byte  [0xb800a], al
        hlt

section .bss
align 4096
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096
stack_bottom:
        ; allocate 4Kib for stack
        resb 4096
stack_top:
