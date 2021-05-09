use peppi::stage::Stage;

// I'd like to apologize for this code.
pub fn name(stage: Stage) -> Option<&'static str> {
    let name = match stage {
        Stage::FOUNTAIN_OF_DREAMS => "FOUNTAIN_OF_DREAMS",
        Stage::POKEMON_STADIUM => "POKEMON_STADIUM",
        Stage::PRINCESS_PEACHS_CASTLE => "PRINCESS_PEACHS_CASTLE",
        Stage::KONGO_JUNGLE => "KONGO_JUNGLE",
        Stage::BRINSTAR => "BRINSTAR",
        Stage::CORNERIA => "CORNERIA",
        Stage::YOSHIS_STORY => "YOSHIS_STORY",
        Stage::ONETT => "ONETT",
        Stage::MUTE_CITY => "MUTE_CITY",
        Stage::RAINBOW_CRUISE => "RAINBOW_CRUISE",
        Stage::JUNGLE_JAPES => "JUNGLE_JAPES",
        Stage::GREAT_BAY => "GREAT_BAY",
        Stage::HYRULE_TEMPLE => "HYRULE_TEMPLE",
        Stage::BRINSTAR_DEPTHS => "BRINSTAR_DEPTHS",
        Stage::YOSHIS_ISLAND => "YOSHIS_ISLAND",
        Stage::GREEN_GREENS => "GREEN_GREENS",
        Stage::FOURSIDE => "FOURSIDE",
        Stage::MUSHROOM_KINGDOM_I => "MUSHROOM_KINGDOM_I",
        Stage::MUSHROOM_KINGDOM_II => "MUSHROOM_KINGDOM_II",
        Stage::VENOM => "VENOM",
        Stage::POKE_FLOATS => "POKE_FLOATS",
        Stage::BIG_BLUE => "BIG_BLUE",
        Stage::ICICLE_MOUNTAIN => "ICICLE_MOUNTAIN",
        Stage::ICETOP => "ICETOP",
        Stage::FLAT_ZONE => "FLAT_ZONE",
        Stage::DREAM_LAND_N64 => "DREAM_LAND_N64",
        Stage::YOSHIS_ISLAND_N64 => "YOSHIS_ISLAND_N64",
        Stage::KONGO_JUNGLE_N64 => "KONGO_JUNGLE_N64",
        Stage::BATTLEFIELD => "BATTLEFIELD",
        Stage::FINAL_DESTINATION => "FINAL_DESTINATION",
        _ => "",
    };

    if name != "" {
        return Some(name);
    } else {
        return None;
    }
}
