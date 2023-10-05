#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused
)]
#![deny(
    deprecated
)]

mod icies;

mod imports;
//mod custom_vars;
//pub mod vars;
/* 
use skyline::libc::c_char;
use skyline::nn::ro::LookupSymbol;
use crate::imports::imports_agent::*;

extern "C"{
    /// gets whether we are in training mode
    #[link_name = "\u{1}_ZN3app9smashball16is_training_modeEv"]
    pub fn is_training_mode() -> bool;
}

//We need the fighter manager solely for the results screen
pub static mut FIGHTER_MANAGER_ADDR: usize = 0;
pub unsafe fn is_Results()-> bool
{
    let mgr = *(FIGHTER_MANAGER_ADDR as *mut *mut app::FighterManager);
    return lua_bind::FighterManager::is_result_mode(mgr);
}
*/
#[skyline::main(name = "smashline_icies_visualizer")]
pub fn main() {
    /* 
    unsafe {
        LookupSymbol(
            &mut FIGHTER_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
                .as_bytes()
                .as_ptr(),
        );
    }*/

    println!("[smashline_icies_visualizer::main] Loading...");
    #[cfg(not(feature = "dev"))]{ 
        icies::install();
    }
    #[cfg(feature = "dev")]{
        icies::install();
    }
    println!("[smashline_icies_visualizer::main] Loaded!");
}