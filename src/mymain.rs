pub fn starter(bootinfo: &'static csnel::BootInfo){
    let (_valid, mapper) = csnel::init(bootinfo);
	csnel::net::sample_deal_all(&mapper);
	// *csnel::net::tcp::TCP_ACCEPT_PORT.lock() = /*put your value here*/;
}
