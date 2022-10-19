use x86_64::{
    VirtAddr,
    structures::{
        idt::{
            InterruptDescriptorTable,
            InterruptStackFrame
        },
        tss::TaskStateSegment,
        gdt::{
            GlobalDescriptorTable,
            Descriptor,
            SegmentSelector
        }
    },
    instructions::{
        tables::load_tss,
        segmentation::{
            CS,
            Segment
        }
    }
};
use lazy_static::lazy_static;

use crate::vga;


#[allow(dead_code)]
type HandlerFunc = extern "x86-interrupt" fn(_: InterruptStackFrame);

const DOUBLE_FAULT_IST_INDEX : u16 = 0;


lazy_static! {

    static ref IDT : InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };

    static ref TSS : TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const      STACK_SIZE  : usize            = 4096 * 5;
            static mut STACK       : [u8; STACK_SIZE] = [0; STACK_SIZE];
            let        stack_start                    = VirtAddr::from_ptr(unsafe {&STACK});
            let        stack_end                      = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };

    static ref GDT : (GlobalDescriptorTable, SegmentSelector, SegmentSelector) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, code_selector, tss_selector)
    };

}


pub fn init() {
    IDT.load();
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1);
        load_tss(GDT.2);
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame : InterruptStackFrame) {
    vga::colour!(LightRed, Black);
    vga::print!("EXCEPTION : BREAKPOINT\n{:#?}\n", stack_frame);
    vga::colour!(White, Black);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame : InterruptStackFrame, _error_code: u64) -> ! {
    vga::colour!(LightRed, Black);
    vga::print!("EXCEPTION : DOUBLE FAULT\n{:#?}", stack_frame);
    vga::colour!(White, Black);
    panic!();
}
