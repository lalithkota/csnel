use csnel::println;

pub fn starter(bootinfo: &'static csnel::BootInfo){
	println!("Hello World{}", "!");
    let (valid, mapper) = csnel::init(bootinfo);
	unsafe{csnel::memory::MAPPER_PTR = &mapper as *const csnel::memory::OffsetPageTable<'static> as u64};
	loop{}
}
