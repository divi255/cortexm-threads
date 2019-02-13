#![no_std]

use core::ptr;

#[repr(C)]
struct ThreadsState {
    // offset of curr and next fields are used by asm code, don't change their position
    curr: u32,
    next: u32,
    // following fields are only used internally
    inited: bool,
    idx: usize,
    add_idx: usize,
    threads: [ThreadControlBlock; 32],
}

#[repr(C)]
pub struct ThreadControlBlock {
    pub sp: u32,
}

pub unsafe extern "C" fn init() {
    __CORTEXM_THREADS_cpsid();
    __CORTEXM_THREADS_GLOBAL.inited = true;
    __CORTEXM_THREADS_GLOBAL_PTR = core::intrinsics::transmute(&__CORTEXM_THREADS_GLOBAL);
    __CORTEXM_THREADS_cpsie();
}

pub unsafe extern "C" fn create_thread(stack: &mut [u32], handler: fn() -> !) {
    __CORTEXM_THREADS_cpsid();
    let idx = stack.len() - 1;
    stack[idx] = 1 << 24;
    stack[idx - 1] = core::intrinsics::transmute(handler as *const fn());
    stack[idx - 2] = 0x0000000E;
    stack[idx - 3] = 0x0000000C;
    stack[idx - 4] = 0x00000003;
    stack[idx - 5] = 0x00000002;
    stack[idx - 6] = 0x00000001;
    stack[idx - 7] = 0x00000000;
    // aditional regs
    stack[idx - 8] = 0x0000000B;
    stack[idx - 9] = 0x0000000A;
    stack[idx - 10] = 0x00000009;
    stack[idx - 11] = 0x00000008;
    stack[idx - 12] = 0x00000007;
    stack[idx - 13] = 0x00000006;
    stack[idx - 14] = 0x00000005;
    stack[idx - 15] = 0x00000004;
    let tcb = ThreadControlBlock {
        sp: core::intrinsics::transmute(&stack[stack.len() - 16]),
    };
    let handler = &mut __CORTEXM_THREADS_GLOBAL;
    handler.threads[handler.add_idx] = tcb;
    handler.add_idx = handler.add_idx + 1;
    __CORTEXM_THREADS_cpsie();
}

#[no_mangle]
pub unsafe extern "C" fn tick() {
    __CORTEXM_THREADS_cpsid();
    let handler = &mut __CORTEXM_THREADS_GLOBAL;
    if handler.inited && handler.add_idx > 0 {
        if handler.curr == handler.next {
            // schedule a thread to be run
            handler.next = core::intrinsics::transmute(&handler.threads[handler.idx]);
            handler.idx = handler.idx + 1;
            if handler.idx >= handler.add_idx {
                handler.idx = 0;
            }
        }
        if handler.curr != handler.next {
            let pend = ptr::read_volatile(0xE000ED04 as *const u32);
            ptr::write_volatile(0xE000ED04 as *mut u32, pend | 1 << 28);
        }
    }
    __CORTEXM_THREADS_cpsie();
}

extern "C" {
    pub fn __CORTEXM_THREADS_PendSVHandler();
    fn __CORTEXM_THREADS_cpsid();
    fn __CORTEXM_THREADS_cpsie();
}

// GLOBALS:
#[no_mangle]
static mut __CORTEXM_THREADS_GLOBAL_PTR: u32 = 0;
static mut __CORTEXM_THREADS_GLOBAL: ThreadsState = ThreadsState {
    curr: 0,
    next: 0,
    inited: false,
    idx: 0,
    add_idx: 0,
    threads: [
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
        ThreadControlBlock { sp: 0 },
    ],
};
