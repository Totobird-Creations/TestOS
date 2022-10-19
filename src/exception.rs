use x86_64::structures::idt::*;


#[repr(C)]
pub struct InterruptDescriptorTable {
    pub divide_by_zero           : Entry<HandlerFunc>,
    pub debug                    : Entry<HandlerFunc>,
    pub non_maskable_interrupt   : Entry<HandlerFunc>,
    pub breakpoint               : Entry<HandlerFunc>,
    pub overflow                 : Entry<HandlerFunc>,
    pub bound_range_exceeded     : Entry<HandlerFunc>,
    pub invalid_opcode           : Entry<HandlerFunc>,
    pub device_not_available     : Entry<HandlerFunc>,
    pub double_fault             : Entry<HandlerFuncWithErrCode>,
    pub invalid_tss              : Entry<HandlerFuncWithErrCode>,
    pub segment_not_present      : Entry<HandlerFuncWithErrCode>,
    pub stack_segment_fault      : Entry<HandlerFuncWithErrCode>,
    pub general_protection_fault : Entry<HandlerFuncWithErrCode>,
    pub page_fault               : Entry<PageFaultHandlerFunc>,
    pub x87_floating_point       : Entry<HandlerFunc>,
    pub alignment_check          : Entry<HandlerFuncWithErrCode>,
    pub machine_check            : Entry<HandlerFunc>,
    pub simd_floating_point      : Entry<HandlerFunc>,
    pub virtualization           : Entry<HandlerFunc>,
    pub security_exception       : Entry<HandlerFuncWithErrCode>
}


type HandlerFunc = extern "x86-interrupt" fn(_: InterruptStackFrame);


pub fn init_idt() {
    let mut idt = InterruptDescriptorTable::new();
}
