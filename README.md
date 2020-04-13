# XReborn
## A port of all the XR ships!
    
## Installation Instructions
    Egosoft will not let us distribute XR assets en mass for
    fear that someone will come along and remake XR. 
    
    So, with permission, we are distributing a program to move
    the XR assets already on your computer into your extensions. 
    We also include xml to define and balance each ship.

    Included is a configuration file called config.toml which 
    includes 3 variables. You may need to edit their values to 
    fit your setup.
    
Your Root Mod Folder:
##### output_path = "C:/Program Files/Steam/steamapps/common/X4 Foundations/Extensions"
Your XR Install:
##### input_path = "C:/Program Files/Steam/steamapps/common/X   Rebirth"
Set False When Updating The Mod to Avoid Extracting Again:
##### extract = true 
    
![ship](https://imgur.com/rVVk1vF)  
![ship](https://imgur.com/GsUeOWs)  
![ship](https://imgur.com/jKwpQnW)  
![ship](https://imgur.com/OWGtlUh)  
![ship](https://imgur.com/c62uqVw)  
![ship](https://imgur.com/kGjz1UO)  
    
- You can find the source [here](https://github.com/albysmith/XReborn-Ships)

## FAQ
- is it save compatible?
  - well it would be but 3.1 has some issue where mods shouldn't support saves
- how do I enter the small and medium ships? 
  - teleport! or use a spacesuit!
- This particular piece of art is terrible!
  - if you make an issue it is likely we can fix it or some friendly blender person will make a replacement
- Why isn't some ship at some faction?
  - we tried to spread them around but if you want them to be available somewhere make an issue on github or nexus to tell us about it
- my ship sinks through the landing pad!
  - because you haven't made an issue on github or nexus yet to tell us about it
- why aren't the numbers exactly the way I want them?
  - because you haven't made an issue on github or nexus yet to tell us about it
- where are the radars?
  - a combination of we forgot and a nasty bug but we can add them if there is demand
- why didn't ego do this? 
  - don't know, they did resuse some of it
- will you do stations/weapons/engines/clusters etc?
  - well we planned to but trying to port the clusters was a nightmare because something is  wrong with some of the shaders from xr [our attempt at the clusters](https://github.com/mewosmith/aa_xr_clusters)
- why doesn't the tool transfer all the XR assets? 
  - nobody has asked yet
- x3 ships?
  - maybe? if you want to help let us know
## Bugs You Don't Even Have to Discover Yourself!
#### Balance
    Balance and bug-fixes are an ongoing process. Please make an issue on github or nexusmods with proposed changes. 
#### Teladi Docks
    Teladi XL have invisible placeholder docks. We have a cool plan for them but wanted to get the ships out faster.
#### Some ships are Temporarily Disabled
    Miners, Liquid Transports, Builders, Resuppliers, and a few unsuitable for player control ships are disabled. They will be enabled shortly. 
#### Some Cockpits/Bridges are Buried
    Yeah, it is hard to check them all ourselves. Make an issue on github or nexusmods
## Contribute to The Project
#### Kit-bashing 
    Having the XR assets neatly packages for you and a large body of examples allows for some very easy ship creation. If you create a kit-bash we would love to include it! 
#### Art Changes
    It is very easy to incorporate your art changes in our distribution. Just let us know and we will help.
#### BalanceMods
    We would be happy to work with you to build and distribute your own balance overhaul or fixes. We have or can quickly make tools to do pretty much anything to these files. 
#### Redistribute Ships Among the Factions
    Happy to help with and distribute anyone's vision of how ships should be allocated to the factions
  
|COMMON NAME|  MACROS |
|---|---|
|Moebius                       |units_size_s_pmc_xen_01_macro,|
|Hayabusa                      |units_size_s_canteran_fighter_01_macro,|
|Orlam                         |units_size_s_ship_01_macro,|
|Talorcan                      |units_size_s_ship_02_macro,|
|Ossian                        |units_size_s_ship_03_macro,|
|Artio                         |units_size_s_ship_04_macro,|
|Vasio                         |units_size_s_ship_05_macro,|
|Nechtan                       |units_size_s_ship_07_macro,|
|Hesus                         |units_size_s_ship_19_macro,|
|Birog                         |units_size_s_ship_23_macro,|
|Camulos Raider                |units_size_s_ship_ar_military_01_macro,|
|Camulos Vanguard              |units_size_s_ship_ar_military_02_macro,|
|Camulos Sentinel              |units_size_s_ship_ar_military_03_macro,|
|Foltor Raider                 |units_size_s_ship_ar_military_04_macro,|
|Foltor Vanguard               |units_size_s_ship_ar_military_05_macro,|
|Foltor Sentinel               |units_size_s_ship_ar_military_06_macro,|
|Eterscel Raider               |units_size_s_ship_ar_military_07_macro,|
|Eterscel Vanguard             |units_size_s_ship_ar_military_08_macro,|
|Eterscel Sentinel             |units_size_s_ship_ar_military_09_macro,|
|Domelch                       |units_size_s_ship_pirate_01_macro,|
|Cennelath                     |units_size_s_ship_pirate_02_macro,|
|Maelchon                      |units_size_s_ship_pirate_03_macro,|
|Triath Raider                 |units_size_s_ship_pmc_01_macro,|
|Triath Vanguard               |units_size_s_ship_pmc_02_macro,|
|Triath Sentinel               |units_size_s_ship_pmc_03_macro,|
|Bonescout                     |units_size_s_split_m3_macro,|
|Skull Crusher                 |units_size_s_split_m4_macro,|
|Falcon                        |units_size_s_te_01_macro,|
|Falcon                        |units_size_s_te_02_macro,|
|Harrier                       |units_size_s_te_03_macro,|
|Harrier                       |units_size_s_te_04_macro,|
|Drostan                       |units_size_s_torpedo_bomber_macro,|
|M                             |units_size_s_xenon_swarm_attack_drone_01_macro,|
|N                             |units_size_s_xenon_swarm_attack_drone_02_macro,|
|Katana                        |units_size_m_canteran_01_macro,|
|Daito                         |units_size_m_canteran_02_macro,|
|Gigurum (Container)           |units_size_m_container_transporter_5_macro,|
|Dwalin                        |units_size_m_crystal_collector_macro,|
|Gigurum (Bulk)                |units_size_m_crystal_transporter_macro,|
|Gigurum (Energy)              |units_size_m_energy_transporter_macro,|
|Nudung                        |units_size_m_hydrogen_collector_macro,|
|Hymir                         |units_size_m_ice_collector_macro,|
|Betaver                       |units_size_m_ions_collector_macro,|
|Gigurum (Liquid)              |units_size_m_liquid_transporter_macro,|
|Mercancias (Liquid)           |units_size_m_mercancias_01_macro,|
|Mercancias (Bulk)             |units_size_m_mercancias_02_macro,|
|Mercancias (Energy)           |units_size_m_mercancias_03_macro,|
|Mercancias (Container)        |units_size_m_mercancias_04_macro,|
|Dwalin                        |units_size_m_nividium_collector_macro,|
|Nyanae (Liquid)               |units_size_m_nyanae_01_macro,|
|Nyanae (Bulk)                 |units_size_m_nyanae_02_macro,|
|Nyanae (Energy)               |units_size_m_nyanae_03_macro,|
|Nyanae (Container)            |units_size_m_nyanae_04_macro,|
|Dwalin                        |units_size_m_ore_collector_macro,|
|Gigurum (Bulk)                |units_size_m_ore_transporter_macro,|
|Betaver                       |units_size_m_plasma_collector_macro,|
|Gigurum (Liquid)              |units_size_m_plasma_transporter_macro,|
|Golem                         |units_size_m_pmc_xen_02_macro,|
|Dwalin                        |units_size_m_silicon_collector_macro,|
|Toucan (Bulk)                 |units_size_m_te_bulk_01_macro,|
|Toucan (Container)            |units_size_m_te_container_01_macro,|
|Toucan (Energy)               |units_size_m_te_energy_01_macro,|
|Manorina                      |units_size_m_te_hydrogen_collector_01_macro,|
|Toucan (Liquid)               |units_size_m_te_liquid_01_macro,|
|Manorina                      |units_size_m_te_mineral_collector_01_macro,|
|Unidentified Flying Object    |units_size_m_ufo_01_macro,|
|Gigurum (Container)           |units_size_m_weapontech_transporter_macro,|
|P                             |units_size_m_xenon_fighter_01_macro,|
|S                             |units_size_m_xenon_miner_01_macro,|
|Zepp                          |units_size_l_ad_zeppelin_01_macro,|
|Onil (Mineral)                |units_size_l_canteran_miner_01_macro,|
|Onil (Gas)                    |units_size_l_canteran_miner_02_macro,|
|Lepton                        |units_size_l_canteran_transporter_macro,|
|Carrier                       |units_size_l_carrier_01_macro,|
|Carrier                       |units_size_l_carrier_02_macro,|
|Fedhelm                       |units_size_l_crystal_collector_macro,|
|Styrvok (Bulk)                |units_size_l_dv_kit_new_bulk_01_macro,|
|Styrvok (Bulk & Liquid)       |units_size_l_dv_kit_new_bulk_liquid_01_macro,|
|Styrvok (Container)           |units_size_l_dv_kit_new_container_01_macro,|
|Stromvok                      |units_size_l_dv_kit_new_defence_01_macro,|
|Styrvok (Energy)              |units_size_l_dv_kit_new_energy_01_macro,|
|Styrvok (Container & Energy)  |units_size_l_dv_kit_new_energy_container_01_macro,|
|Styrvok (Liquid)              |units_size_l_dv_kit_new_liquid_01_macro,|
|Styrvok (Bulk)                |units_size_l_dv_kit_old_bulk_01_macro,|
|Styrvok (Bulk & Liquid)       |units_size_l_dv_kit_old_bulk_liquid_01_macro,|
|Styrvok (Container)           |units_size_l_dv_kit_old_container_01_macro,|
|Stromvok                      |units_size_l_dv_kit_old_defence_01_macro,|
|Styrvok (Energy)              |units_size_l_dv_kit_old_energy_01_macro,|
|Styrvok (Container & Energy)  |units_size_l_dv_kit_old_energy_container_01_macro,|
|Styrvok (Liquid)              |units_size_l_dv_kit_old_liquid_01_macro,|
|Boann                         |units_size_l_hydrogen_collector_macro,|
|Sequana                       |units_size_l_ice_collector_macro,|
|Midir                         |units_size_l_ions_collector_macro,|
|Rahanas (Bulk)                |units_size_l_kit_bulk_01_macro,|
|Light Sul                     |units_size_l_kit_carrier_01_macro,|
|Heavy Sul                     |units_size_l_kit_carrier_02_macro,|
|Rahanas (Container)           |units_size_l_kit_container_01_macro,|
|Rahanas (Container)           |units_size_l_kit_container_02_macro,|
|Sanahar (Container)           |units_size_l_kit_container_03_macro,|
|Rahanas (Energy)              |units_size_l_kit_energy_01_macro,|
|Rahanas (Bulk & Liquid)       |units_size_l_kit_hybrid_01_macro,|
|Rahanas (Container & Energy)  |units_size_l_kit_hybrid_02_macro,|
|Rahanas (Liquid)              |units_size_l_kit_liquid_01_macro,|
|Hermod (Bulk & Liquid)        |units_size_l_liquid_freighter_macro,|
|Fedhelm                       |units_size_l_nividium_collector_macro,|
|Fedhelm                       |units_size_l_ore_collector_macro,|
|Midir                         |units_size_l_plasma_collector_macro,|
|Fedhelm                       |units_size_l_silicon_collector_macro,|
|Balor                         |units_size_l_single_attack_ship_macro,|
|Vulture (Bulk)                |units_size_l_te_kit_bulk_03_macro,|
|Vulture (Container)           |units_size_l_te_kit_container_03_macro,|
|Vulture (Energy)              |units_size_l_te_kit_energy_03_macro,|
|Vulture (Liquid)              |units_size_l_te_kit_liquid_03_macro,|
|K                             |units_size_l_xenon_01_macro,|
|Construction Vessel           |units_size_xl_builder_ship_dv_macro,|
|Construction Vessel           |units_size_xl_builder_ship_hol_macro,|
|Construction Vessel           |units_size_xl_builder_ship_macro,|
|Construction Vessel           |units_size_xl_builder_ship_ol_macro,|
|Construction Vessel           |units_size_xl_builder_ship_plot_01_macro,|
|Arawn                         |units_size_xl_capital_destroyer_1_macro,|
|Taranis                       |units_size_xl_capital_destroyer_2_macro,|
|Titurel                       |units_size_xl_cargo_hauler_2_macro,|
|Scaldis                       |units_size_xl_cargo_hauler_3_macro,|
|Lyranea                       |units_size_xl_cs_omicron_ship_01_macro,|
|Fulmekron                     |units_size_xl_cs_omicron_ship_02_macro,|
|Olmekron                      |units_size_xl_cs_omicron_ship_03_macro,|
|Lyramekron                    |units_size_xl_cs_omicron_ship_04_macro,|
|Sucellus                      |units_size_xl_red_destroyer_macro,|
|Gangrene Chaser               |units_size_xl_split_m1_macro,|
|Albatross (Bulk)              |units_size_xl_te_kit_bulk_01_macro,|
|Albatross (Bulk)              |units_size_xl_te_kit_bulk_02_macro,|
|Crane (Bulk)                  |units_size_xl_te_kit_bulk_03_macro,|
|Albatross (Container)         |units_size_xl_te_kit_container_01_macro,|
|Albatross (Container)         |units_size_xl_te_kit_container_02_macro,|
|Crane (Container)             |units_size_xl_te_kit_container_03_macro,|
|Albatross (Energy)            |units_size_xl_te_kit_energy_01_macro,|
|Albatross (Energy)            |units_size_xl_te_kit_energy_02_macro,|
|Crane (Energy)                |units_size_xl_te_kit_energy_03_macro,|
|Albatross (Liquid)            |units_size_xl_te_kit_liquid_01_macro,|
|Albatross (Liquid)            |units_size_xl_te_kit_liquid_02_macro,|
|Crane (Liquid)                |units_size_xl_te_kit_liquid_03_macro,|
|Phoenix                       |units_size_xl_te_kit_none_01_macro,|
|Condor                        |units_size_xl_te_kit_none_02_macro,|
|Phoenix                       |units_size_xl_te_kit_none_03_macro,|
|Super-Freighter               |units_size_xl_transporter_1_macro,|
|Super-Freighter               |units_size_xl_transporter_2_macro,|
|Super-Freighter               |units_size_xl_transporter_3_macro,|
|I                             |units_size_xl_xenon_01_macro,|

### Special Thanks To:
  - [discord](https://discord.gg/AWNAKU9) (seriously, practically everyone has contributed in one way or another)
  - egosoft
  - unitrader
  - cg089
  - h20dragon
  - deadair
  - brummbear
  - wookiee
  - macbain

![ship](https://ibb.co/cxv0rr9) 
