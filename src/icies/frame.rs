use crate::imports::imports_agent::*;
pub static mut popo_sphere_id: [u64; 8] = [0; 8];

unsafe fn nana_indicator(fighter: &mut L2CFighterCommon, boma: *mut BattleObjectModuleAccessor, fighter_kind: i32) {
    let entry = get_entry(fighter) as usize;
    if fighter_kind == *FIGHTER_KIND_POPO {
        if LinkModule::is_link(fighter.module_accessor, *FIGHTER_POPO_LINK_NO_PARTNER) {
            let my_nana_id = LinkModule::get_parent_id(fighter.module_accessor, *FIGHTER_POPO_LINK_NO_PARTNER,false) as u32;

            if smash::app::sv_battle_object::is_active(my_nana_id) {
                let nana = sv_battle_object::module_accessor(my_nana_id);

                let nana_opt_offset = WorkModule::get_param_float(boma, smash::hash40("param_private"), smash::hash40("nana_pos_x_offset"));
                let nana_opt_dst_max = WorkModule::get_param_float(boma, smash::hash40("param_private"), smash::hash40("nana_opt_dst_max"));
                let nana_opt_dst_near = WorkModule::get_param_float(boma, smash::hash40("param_private"), smash::hash40("nana_opt_near_dst"));

                let nana_pos = *PostureModule::pos(nana);
                let pos = *PostureModule::pos(boma);
                let lr = PostureModule::lr(boma);

                let distance = sv_math::vec2_distance(pos.x+nana_opt_offset*lr,pos.y,nana_pos.x, nana_pos.y);
                //println!("Distance: {distance}");
                
                let nana_color_b = Vector4f{x: 204.0/255.0, y: 67.0/255.0, z: 109.0/255.0, w: 0.2};
                let nana_color_d = Vector4f{x: 204.0/255.0, y: 67.0/255.0, z: 109.0/255.0, w: 0.8};
                if distance <= nana_opt_dst_near {
                    ColorBlendModule::set_main_color(nana, &nana_color_b, &nana_color_d, 0.21, 2.2, 3, true);
                }
                else {
                    ColorBlendModule::cancel_main_color(nana, 0);
                }

                if !EffectModule::is_exist_effect(boma, popo_sphere_id[entry] as u32) {
                    let size_mult = 19.0 / 200.0; //thanks jugeeya
                    EFFECT_FOLLOW(fighter,Hash40::new("sys_shield_damage3"),Hash40::new("top"),
                    0,1.0,nana_opt_offset,
                    0,0,0,
                    nana_opt_dst_near*size_mult,false);

                    popo_sphere_id[entry] = EffectModule::get_last_handle(boma);
                }
            }
        }
    }
}
unsafe fn icy_update(fighter: &mut L2CFighterCommon, fighter_kind: i32){
    nana_indicator(fighter,fighter.module_accessor,fighter_kind);
}

#[fighter_frame( agent = FIGHTER_KIND_POPO )]
fn popo_update(fighter: &mut L2CFighterCommon) {
    unsafe {
        icy_update(fighter,*FIGHTER_KIND_POPO);
    }
}
#[fighter_frame( agent = FIGHTER_KIND_NANA )]
fn nana_update(fighter: &mut L2CFighterCommon) {
    unsafe {
        icy_update(fighter,*FIGHTER_KIND_NANA);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        nana_update,
        popo_update,
    );
}