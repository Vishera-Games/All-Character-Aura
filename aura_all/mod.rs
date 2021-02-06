use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};

static mut _TIME_COUNTER: [i32; 8] = [0; 8]; //FRAME COUNTER FOR GFX TIMING

pub fn once_per_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        //let status_kind = StatusModule::status_kind(module_accessor);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let kind = smash::app::utility::get_kind(module_accessor);
	let category = smash::app::utility::get_category(module_accessor);
        //let motion_kind = MotionModule::motion_kind(module_accessor);
        //let motion_frame = MotionModule::frame(module_accessor);

	let gfxcoords  = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
	let mut gfxsize: [f32; 8] = [0.0; 8];
	let mut maxgfxsize: [f32; 8] = [0.28; 8]; //Every GFX size is different so couldn't use one set value
	let mut currentpercent: [f32; 8] = [0.0; 8]; //Not used was trying to get some projectiles to work but failed
	let mut countermax: [i32; 8] = [8; 8]; //THE MAX VALUE THE GFX FRAME COUNTER WILL HIT BEFORE RESETTING TO 0
	let mut gfxname: [&str; 8] = ["sys_hit_aura"; 8]; //The actual name of the GFX used
          
        if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		if kind == *FIGHTER_KIND_GANON { //I AM SEPARATING CHARACTERS INTO WHICH GFX I WANT THEIR AURA TO USE, AS WELL AS SETTING THE VALUES FOR MAX GFX SIZE AND GFX FRAME COUNTER
			maxgfxsize[entry_id] = 1.5; //Setting the max GFX size for these specific characters, every GFX is different
			gfxname[entry_id] = "ganon_attack_purple"; //Setting the name of the GFX used here
			countermax[entry_id] = 3; //This means the game will generate the aura GFX every 3 frames, this one specifically is short so it needed a smaller gap
		}
		if kind == *FIGHTER_KIND_MARIO || kind == *FIGHTER_KIND_SAMUS || kind == *FIGHTER_KIND_LUIGI || kind == *FIGHTER_KIND_NESS || kind == *FIGHTER_KIND_CAPTAIN || kind == *FIGHTER_KIND_PEACH || kind == *FIGHTER_KIND_DAISY || kind == *FIGHTER_KIND_KOOPA || kind == *FIGHTER_KIND_ZELDA || kind == *FIGHTER_KIND_ROY || kind == *FIGHTER_KIND_IKE || kind == *FIGHTER_KIND_LIZARDON || kind == *FIGHTER_KIND_ROCKMAN || kind == *FIGHTER_KIND_LITTLEMAC || kind == *FIGHTER_KIND_KEN || kind == *FIGHTER_KIND_GAOGAEN || kind == *FIGHTER_KIND_PACKUN || kind == *FIGHTER_KIND_DOLLY {
			maxgfxsize[entry_id] = 0.5;
			gfxname[entry_id] = "sys_hit_fire"; //FOR FIGHTERS WITH FIRE AURA
			countermax[entry_id] = 6;
		}
		if kind == *FIGHTER_KIND_SAMUSD || kind == *FIGHTER_KIND_PURIN || kind == *FIGHTER_KIND_PICHU || kind == *FIGHTER_KIND_MEWTWO || kind == *FIGHTER_KIND_PIKMIN || kind == *FIGHTER_KIND_MURABITO || kind == *FIGHTER_KIND_REFLET || kind == *FIGHTER_KIND_BAYONETTA || kind == *FIGHTER_KIND_RIDLEY || kind == *FIGHTER_KIND_SHIZUE || kind == *FIGHTER_KIND_PICKEL || kind == *FIGHTER_KIND_EDGE {
			maxgfxsize[entry_id] = 0.3;
			gfxname[entry_id] = "sys_hit_purple"; //FOR FIGHTERS WITH DARK AURA
			countermax[entry_id] = 6;
		}
		if kind == *FIGHTER_KIND_PIKACHU || kind == *FIGHTER_KIND_MARIOD || kind == *FIGHTER_KIND_PICHU || kind == *FIGHTER_KIND_GAMEWATCH || kind == *FIGHTER_KIND_ROBOT {
			maxgfxsize[entry_id] = 0.45;
			gfxname[entry_id] = "sys_hit_elec"; //FOR FIGHTERS WITH ELECTRIC AURA
			countermax[entry_id] = 8;
		}
            	if DamageModule::damage(module_accessor, 0) < 65.0 {
			gfxsize[entry_id] = 0.0;
			AttackModule::set_power_up(module_accessor, (66.0 + 0.523076923 * DamageModule::damage(module_accessor, 0)) / 100.0);
			//If your damage is less than 65, apply this formula to your attack power. Used Lucarios actual formulas
		}
		if DamageModule::damage(module_accessor, 0) == 65.0 {
			gfxsize[entry_id] = DamageModule::damage(module_accessor, 0) * (maxgfxsize[entry_id] / 225.0);
			AttackModule::set_power_up(module_accessor, 1.0);
			//If your damage is 65, set your power multiplier to 1.0 like Lucario's
		}
		if DamageModule::damage(module_accessor, 0) > 65.0 { //IF YOUR DAMAGE IS BIGGER THAN 65
			_TIME_COUNTER[entry_id] += 1; //GFX TIMER STARTS COUNTING
			if DamageModule::damage(module_accessor, 0) > 225.0 { //If damage is greater than 225
				gfxsize[entry_id] = maxgfxsize[entry_id]; //This 225.0 is the percentage you will reach your max aura. I set the gfx to stop scaling beyond this and just use their flat max values
			}
			if DamageModule::damage(module_accessor, 0) < 225.0 { //If your damage is less than 225
				gfxsize[entry_id] = DamageModule::damage(module_accessor, 0) * (maxgfxsize[entry_id] / 225.0); //Applies the scaling formula to your gfx. It's simply just the max size divided by the total scale window for percentage (0% - 225%) and then multiplied by your current percentage
			}
			AttackModule::set_power_up(module_accessor, (100.0 + 0.48 * (DamageModule::damage(module_accessor, 0) - 65.0)) / 100.0); //Applies the 65%+ Lucario formula to attack power 
			if _TIME_COUNTER[entry_id] >= countermax[entry_id] { //If the character's GFX timer reaches their max value, it will draw the GFX
				EffectModule::req_follow(module_accessor, smash::phx::Hash40::new(gfxname[entry_id]), smash::phx::Hash40::new("haver"), &gfxcoords, &gfxcoords, gfxsize[entry_id], true, 0, 0, 0, 0, 0, true, true);
				EffectModule::req_follow(module_accessor, smash::phx::Hash40::new(gfxname[entry_id]), smash::phx::Hash40::new("havel"), &gfxcoords, &gfxcoords, gfxsize[entry_id], true, 0, 0, 0, 0, 0, true, true);
				_TIME_COUNTER[entry_id] = 0; //Since the timer has reached the max count, it will just keep going up past it and activate the gfx every frame so we have to reset it to 0
			}
		}
        }
    }
}

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
    acmd::add_hooks!(
    );
}
