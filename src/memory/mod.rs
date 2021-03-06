use x86_64::VirtAddr;

pub use x86_64::structures::paging::MapperAllSizes;
pub use x86_64::structures::paging::PageTable;
pub use x86_64::structures::paging::OffsetPageTable;

pub unsafe fn init(phy_mem_offset: u64 ) -> OffsetPageTable<'static> {
    let physical_memory_offset=VirtAddr::new(phy_mem_offset);
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}
