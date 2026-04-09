use std::arch::asm;

// Sets EAX to the leaf passed in. Then calls CPUID and returns resulting register values
// Push and pop of rbx are required because LLVM treats RBX as a restricted callee-saved register
// inout("eax") sets EAX to leaf and also outputs its value at the end of the asm to eax variable
pub fn cpuid(leaf: u32) -> (u32, u32, u32, u32) {
    let eax: u32;
    let ebx: u32;
    let ecx: u32;
    let edx: u32;
    unsafe {
        asm!(
            "push rbx",
            "cpuid",
            "mov {0:e}, ebx",
            "pop rbx",
            out(reg) ebx,
            inout("eax") leaf=>eax,
            out("ecx") ecx,
            out("edx") edx,
        );
    }
    (eax, ebx, ecx, edx)
}

// Sets ecx to index, calls read MSR, returns concatenation of edx:eax
pub fn rdmsr(index: u32) -> u64 {
    let eax: u32;
    let edx: u32;
    unsafe {
        asm!(
            "rdmsr",
            in("ecx") index,
            out("eax") eax,
            out("edx") edx,
        );
    }
    (edx << 32 | eax).into()
}

// Sets ecx to index, sets edx:eax to upper:lower of val, then calls write MSR
pub fn wrmsr(index: u32, val: u64) {
    let edx: u32 = (val >> 32) as u32;
    let eax: u32 = val as u32;
    unsafe {
        asm!(
            "wrmsr",
            in("ecx") index,
            in("eax") eax,
            in("edx") edx,
        );
    }
}
