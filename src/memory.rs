use x86_64::{
    structures::paging::{
        Page,
        PhysFrame,
        Mapper,
        Size4KiB,
        FrameAllocator,
        PageTable,
        OffsetPageTable
    },
    VirtAddr,
    PhysAddr,
};

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// 有効なレベル4テーブルへのミュータブル参照を返す。
///
/// この関数はunsafeである。なぜなら、渡された`physical_memory_offset`を
/// complete物理メモリが仮想メモリにマッピングさることをcallerが保証する
/// 必要があるからである。また、この関数は `&mut`参照のエイリアス化を避けるために
/// 一度しか呼んではなりません(複数回の呼び出しは未定義動作です)。
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
    -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (lebel_4_table_frame, _) = Cr3::read();

    let phys = lebel_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr    // unsafe
}

pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        // FIXME: this is not safe, we do it only for testing
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}