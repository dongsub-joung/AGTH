extern crate winapi;

use std::ptr;
use winapi::um::memoryapi::VirtualProtect;
use winapi::um::winnt::PAGE_EXECUTE_READWRITE;

pub fn hook_function(target_address: *mut u8, hook_function: fn()) {
    unsafe {
        let mut old_protect: u32 = 0;

        // Change the memory protection
        VirtualProtect(target_address, 5, PAGE_EXECUTE_READWRITE, &mut old_protect);

        // Write your hook
        *(target_address as *mut u8) = 0xE9; // JMP instruction
        *(target_address.add(1) as *mut u32) = (hook_function as *const fn() as u32) - (target_address as u32 + 5);

        // Restore old protection (if necessary)
        VirtualProtect(target_address, 5, old_protect, &mut old_protect);
    }
}

// Your hook function
pub fn my_hook() -> Box<String> {
    println!("Hooked!");
    
    Box::from("String".to_string())
}