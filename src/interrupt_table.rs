#[repr(C)]
struct Entry<T> {
    fp1: u16,
    gdt_sel: u16,
    opt: Options,
    fp2: u16,
    fp3: u32,
    _reserved: u32,
    _phantom: core::marker::PhantomData<T>,
}

type Handler = extern "x86-interrupt" fn(_: &mut InterruptStackFrame);

#[repr(transparent)]
struct Options(u16);

pub struct InterruptTable {
    pub divide_by_zero: Entry<Handler>,
    pub debug: Entry<Handler>,
    pub non_maskeble_interrupt: Entry<Handler>,
    pub breakpoint: Entry<Handler>,
    pub overflow: Entry<Handler>,
    pub bound_range_exceeded: Entry<Handler>,
    pub invalid_opcode: Entry<Handler>,
    pub device_not_available: Entry<Handler>,
    pub double_fault: Entry<Handler>,
    pub _padding: Entry<Handler>,
    pub invalidTSS: Entry<Handler>,
    pub segment_not_present: Entry<Handler>,
    pub stack_segment_fault: Entry<Handler>,
    pub general_protection_fault: Entry<Handler>,
    pub page_fault: Entry<Handler>,
    pub _reserved0: Entry<Handler>,
    pub x87_floating_point_exception: Entry<Handler>,
    pub alignment_check: Entry<Handler>,
    pub machine_check: Entry<Handler>,
    pub simd_floating_point_exception: Entry<Handler>,
    pub virtualization_exception: Entry<Handler>,
    pub _reserved1: Entry<Handler>,
    pub security_exception: Entry<Handler>,
    pub _reserved2: Entry<Handler>,
}

