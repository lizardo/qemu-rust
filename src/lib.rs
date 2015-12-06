fn init() {
    println!("Hello world");
}

#[no_mangle]
pub extern fn qemu_stamp_839da99cb11c946052ea582b1c20c67d8a604d28() {}
#[no_mangle]
pub extern fn qemu_module_dummy() {}

#[link_section = ".init_array"]
#[allow(dead_code)]
static NEXUS6_MACHINE_INIT: fn() = init;
