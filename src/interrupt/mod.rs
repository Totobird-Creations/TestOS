use core::iter::Chain;

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
        },
        interrupts
    },
};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;

mod breakpoint;
mod double_fault;
mod timer;
mod keyboard;


#[allow(dead_code)]
type HandlerFunc = extern "x86-interrupt" fn(_: InterruptStackFrame);


lazy_static! {

    static ref IDT : InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        breakpoint   ::setup(&mut idt);
        double_fault ::setup(&mut idt);
        timer        ::setup(&mut idt);
        keyboard     ::setup(&mut idt);
        idt
    };

    static ref TSS : TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[double_fault::DOUBLE_FAULT_IST_INDEX as usize] = {
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

const  PIC_1_OFFSET : u8                 = 32;
const  PIC_2_OFFSET : u8                 = PIC_1_OFFSET + 8;
static PICS         : Mutex<ChainedPics> = Mutex::new(unsafe {ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)});


#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer    = PIC_1_OFFSET,
    Keyboard
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        return self as u8;
    }
    fn as_usize(self) -> usize {
        return usize::from(self.as_u8());
    }
}


pub fn init() {
    IDT.load();
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1);
        load_tss(GDT.2);
    }
    unsafe {
        PICS.lock().initialize();
    }
    interrupts::enable();
}
