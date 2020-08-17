use x86_64::{
    structures::paging::{PageTable, OffsetPageTable},
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
