fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    const TYPES: [&str; 18] = [
        "Bug", "Dark", "Dragon", "Electric", "Fairy", "Fighting", "Fire", "Flying", "Ghost",
        "Grass", "Ground", "Ice", "Normal", "Poison", "Psychic", "Rock", "Steel", "Water",
    ];
    #[rustfmt::skip]
    const POKEMONS: [(&str, usize, i32); 718] = [
        ("Bulbasaur", 9, 13), ("Ivysaur", 9, 13), ("Venusaur", 9, 13), ("Charmander", 6, -1), ("Charmeleon", 6, -1), ("Charizard", 6, 7), ("Squirtle", 17, -1), ("Wartortle", 17, -1), ("Blastoise", 17, -1), ("Caterpie", 0, -1), ("Metapod", 0, -1), ("Butterfree", 0, 7), ("Weedle", 0, 13), ("Kakuna", 0, 13), ("Beedrill", 0, 13), ("Pidgey", 12, 7), ("Pidgeotto", 12, 7), ("Pidgeot", 12, 7), ("Rattata", 12, -1), ("Raticate", 12, -1), ("Spearow", 12, 7), ("Fearow", 12, 7), ("Ekans", 13, -1), ("Arbok", 13, -1), ("Pikachu", 3, -1), ("Raichu", 3, -1), ("Sandshrew", 10, -1), ("Sandslash", 10, -1), ("Nidoran♀", 13, -1), ("Nidorina", 13, -1), ("Nidoqueen", 13, 10), ("Nidoran♂", 13, -1), ("Nidorino", 13, -1), ("Nidoking", 13, 10), ("Clefairy", 4, -1), ("Clefable", 4, -1), ("Vulpix", 6, -1), ("Ninetales", 6, -1), ("Jigglypuff", 12, 4), ("Wigglytuff", 12, 4), ("Zubat", 13, 7), ("Golbat", 13, 7), ("Oddish", 9, 13), ("Gloom", 9, 13), ("Vileplume", 9, 13), ("Paras", 0, 9), ("Parasect", 0, 9), ("Venonat", 0, 13), ("Venomoth", 0, 13), ("Diglett", 10, -1), ("Dugtrio", 10, -1), ("Meowth", 12, -1), ("Persian", 12, -1), ("Psyduck", 17, -1), ("Golduck", 17, -1), ("Mankey", 5, -1), ("Primeape", 5, -1), ("Growlithe", 6, -1), ("Arcanine", 6, -1), ("Poliwag", 17, -1), ("Poliwhirl", 17, -1), ("Poliwrath", 17, 5), ("Abra", 14, -1), ("Kadabra", 14, -1), ("Alakazam", 14, -1), ("Machop", 5, -1), ("Machoke", 5, -1), ("Machamp", 5, -1), ("Bellsprout", 9, 13), ("Weepinbell", 9, 13), ("Victreebel", 9, 13), ("Tentacool", 17, 13), ("Tentacruel", 17, 13), ("Geodude", 15, 10), ("Graveler", 15, 10), ("Golem", 15, 10), ("Ponyta", 6, -1), ("Rapidash", 6, -1), ("Slowpoke", 17, 14), ("Slowbro", 17, 14), ("Magnemite", 3, 16), ("Magneton", 3, 16), ("Farfetch'd", 12, 7), ("Doduo", 12, 7), ("Dodrio", 12, 7), ("Seel", 17, -1), ("Dewgong", 17, 11), ("Grimer", 13, -1), ("Muk", 13, -1), ("Shellder", 17, -1), ("Cloyster", 17, 11), ("Gastly", 8, 13), ("Haunter", 8, 13), ("Gengar", 8, 13), ("Onix", 15, 10), ("Drowzee", 14, -1), ("Hypno", 14, -1), ("Krabby", 17, -1), ("Kingler", 17, -1), ("Voltorb", 3, -1), ("Electrode", 3, -1), ("Exeggcute", 9, 14), ("Exeggutor", 9, 14), ("Cubone", 10, -1), ("Marowak", 10, -1), ("Hitmonlee", 5, -1), ("Hitmonchan", 5, -1), ("Lickitung", 12, -1), ("Koffing", 13, -1), ("Weezing", 13, -1), ("Rhyhorn", 10, 15), ("Rhydon", 10, 15), ("Chansey", 12, -1), ("Tangela", 9, -1), ("Kangaskhan", 12, -1), ("Horsea", 17, -1), ("Seadra", 17, -1), ("Goldeen", 17, -1), ("Seaking", 17, -1), ("Staryu", 17, -1), ("Starmie", 17, 14), ("Mr. Mime", 14, 4), ("Scyther", 0, 7), ("Jynx", 11, 14), ("Electabuzz", 3, -1), ("Magmar", 6, -1), ("Pinsir", 0, -1), ("Tauros", 12, -1), ("Magikarp", 17, -1), ("Gyarados", 17, 7), ("Lapras", 17, 11), ("Ditto", 12, -1), ("Eevee", 12, -1), ("Vaporeon", 17, -1), ("Jolteon", 3, -1), ("Flareon", 6, -1), ("Porygon", 12, -1), ("Omanyte", 15, 17), ("Omastar", 15, 17), ("Kabuto", 15, 17), ("Kabutops", 15, 17), ("Aerodactyl", 15, 7), ("Snorlax", 12, -1), ("Articuno", 11, 7), ("Zapdos", 3, 7), ("Moltres", 6, 7), ("Dratini", 2, -1), ("Dragonair", 2, -1), ("Dragonite", 2, 7), ("Mewtwo", 14, -1), ("Mew", 14, -1), ("Chikorita", 9, -1), ("Bayleef", 9, -1), ("Meganium", 9, -1), ("Cyndaquil", 6, -1), ("Quilava", 6, -1), ("Typhlosion", 6, -1), ("Totodile", 17, -1), ("Croconaw", 17, -1), ("Feraligatr", 17, -1), ("Sentret", 12, -1), ("Furret", 12, -1), ("Hoothoot", 12, 7), ("Noctowl", 12, 7), ("Ledyba", 0, 7), ("Ledian", 0, 7), ("Spinarak", 0, 13), ("Ariados", 0, 13), ("Crobat", 13, 7), ("Chinchou", 17, 3), ("Lanturn", 17, 3), ("Pichu", 3, -1), ("Cleffa", 4, -1), ("Igglybuff", 12, 4), ("Togepi", 4, -1), ("Togetic", 4, 7), ("Natu", 14, 7), ("Xatu", 14, 7), ("Mareep", 3, -1), ("Flaaffy", 3, -1), ("Ampharos", 3, -1), ("Bellossom", 9, -1), ("Marill", 17, 4), ("Azumarill", 17, 4), ("Sudowoodo", 15, -1), ("Politoed", 17, -1), ("Hoppip", 9, 7), ("Skiploom", 9, 7), ("Jumpluff", 9, 7), ("Aipom", 12, -1), ("Sunkern", 9, -1), ("Sunflora", 9, -1), ("Yanma", 0, 7), ("Wooper", 17, 10), ("Quagsire", 17, 10), ("Espeon", 14, -1), ("Umbreon", 1, -1), ("Murkrow", 1, 7), ("Slowking", 17, 14), ("Misdreavus", 8, -1), ("Unown", 14, -1), ("Wobbuffet", 14, -1), ("Girafarig", 12, 14), ("Pineco", 0, -1), ("Forretress", 0, 16), ("Dunsparce", 12, -1), ("Gligar", 10, 7), ("Steelix", 16, 10), ("Snubbull", 4, -1), ("Granbull", 4, -1), ("Qwilfish", 17, 13), ("Scizor", 0, 16), ("Shuckle", 0, 15), ("Heracross", 0, 5), ("Sneasel", 1, 11), ("Teddiursa", 12, -1), ("Ursaring", 12, -1), ("Slugma", 6, -1), ("Magcargo", 6, 15), ("Swinub", 11, 10), ("Piloswine", 11, 10), ("Corsola", 17, 15), ("Remoraid", 17, -1), ("Octillery", 17, -1), ("Delibird", 11, 7), ("Mantine", 17, 7), ("Skarmory", 16, 7), ("Houndour", 1, 6), ("Houndoom", 1, 6), ("Kingdra", 17, 2), ("Phanpy", 10, -1), ("Donphan", 10, -1), ("Porygon2", 12, -1), ("Stantler", 12, -1), ("Smeargle", 12, -1), ("Tyrogue", 5, -1), ("Hitmontop", 5, -1), ("Smoochum", 11, 14), ("Elekid", 3, -1), ("Magby", 6, -1), ("Miltank", 12, -1), ("Blissey", 12, -1), ("Raikou", 3, -1), ("Entei", 6, -1), ("Suicune", 17, -1), ("Larvitar", 15, 10), ("Pupitar", 15, 10), ("Tyranitar", 15, 1), ("Lugia", 14, 7), ("Ho-oh", 6, 7), ("Celebi", 14, 9), ("Treecko", 9, -1), ("Grovyle", 9, -1), ("Sceptile", 9, -1), ("Torchic", 6, -1), ("Combusken", 6, 5), ("Blaziken", 6, 5), ("Mudkip", 17, -1), ("Marshtomp", 17, 10), ("Swampert", 17, 10), ("Poochyena", 1, -1), ("Mightyena", 1, -1), ("Zigzagoon", 12, -1), ("Linoone", 12, -1), ("Wurmple", 0, -1), ("Silcoon", 0, -1), ("Beautifly", 0, 7), ("Cascoon", 0, -1), ("Dustox", 0, 13), ("Lotad", 17, 9), ("Lombre", 17, 9), ("Ludicolo", 17, 9), ("Seedot", 9, -1), ("Nuzleaf", 9, 1), ("Shiftry", 9, 1), ("Taillow", 12, 7), ("Swellow", 12, 7), ("Wingull", 17, 7), ("Pelipper", 17, 7), ("Ralts", 14, 4), ("Kirlia", 14, 4), ("Gardevoir", 14, 4), ("Surskit", 0, 17), ("Masquerain", 0, 7), ("Shroomish", 9, -1), ("Breloom", 9, 5), ("Slakoth", 12, -1), ("Vigoroth", 12, -1), ("Slaking", 12, -1), ("Nincada", 0, 10), ("Ninjask", 0, 7), ("Shedinja", 0, 8), ("Whismur", 12, -1), ("Loudred", 12, -1), ("Exploud", 12, -1), ("Makuhita", 5, -1), ("Hariyama", 5, -1), ("Azurill", 12, 4), ("Nosepass", 15, -1), ("Skitty", 12, -1), ("Delcatty", 12, -1), ("Sableye", 1, 8), ("Mawile", 16, 4), ("Aron", 16, 15), ("Lairon", 16, 15), ("Aggron", 16, 15), ("Meditite", 5, 14), ("Medicham", 5, 14), ("Electrike", 3, -1), ("Manectric", 3, -1), ("Plusle", 3, -1), ("Minun", 3, -1), ("Volbeat", 0, -1), ("Illumise", 0, -1), ("Roselia", 9, 13), ("Gulpin", 13, -1), ("Swalot", 13, -1), ("Carvanha", 17, 1), ("Sharpedo", 17, 1), ("Wailmer", 17, -1), ("Wailord", 17, -1), ("Numel", 6, 10), ("Camerupt", 6, 10), ("Torkoal", 6, -1), ("Spoink", 14, -1), ("Grumpig", 14, -1), ("Spinda", 12, -1), ("Trapinch", 10, -1), ("Vibrava", 10, 2), ("Flygon", 10, 2), ("Cacnea", 9, -1), ("Cacturne", 9, 1), ("Swablu", 12, 7), ("Altaria", 2, 7), ("Zangoose", 12, -1), ("Seviper", 13, -1), ("Lunatone", 15, 14), ("Solrock", 15, 14), ("Barboach", 17, 10), ("Whiscash", 17, 10), ("Corphish", 17, -1), ("Crawdaunt", 17, 1), ("Baltoy", 10, 14), ("Claydol", 10, 14), ("Lileep", 15, 9), ("Cradily", 15, 9), ("Anorith", 15, 0), ("Armaldo", 15, 0), ("Feebas", 17, -1), ("Milotic", 17, -1), ("Castform", 12, -1), ("Kecleon", 12, -1), ("Shuppet", 8, -1), ("Banette", 8, -1), ("Duskull", 8, -1), ("Dusclops", 8, -1), ("Tropius", 9, 7), ("Chimecho", 14, -1), ("Absol", 1, -1), ("Wynaut", 14, -1), ("Snorunt", 11, -1), ("Glalie", 11, -1), ("Spheal", 11, 17), ("Sealeo", 11, 17), ("Walrein", 11, 17), ("Clamperl", 17, -1), ("Huntail", 17, -1), ("Gorebyss", 17, -1), ("Relicanth", 17, 15), ("Luvdisc", 17, -1), ("Bagon", 2, -1), ("Shelgon", 2, -1), ("Salamence", 2, 7), ("Beldum", 16, 14), ("Metang", 16, 14), ("Metagross", 16, 14), ("Regirock", 15, -1), ("Regice", 11, -1), ("Registeel", 16, -1), ("Latias", 2, 14), ("Latios", 2, 14), ("Kyogre", 17, -1), ("Groudon", 10, -1), ("Rayquaza", 2, 7), ("Jirachi", 16, 14), ("Deoxys", 14, -1), ("Turtwig", 9, -1), ("Grotle", 9, -1), ("Torterra", 9, 10), ("Chimchar", 6, -1), ("Monferno", 6, 5), ("Infernape", 6, 5), ("Piplup", 17, -1), ("Prinplup", 17, -1), ("Empoleon", 17, 16), ("Starly", 12, 7), ("Staravia", 12, 7), ("Staraptor", 12, 7), ("Bidoof", 12, -1), ("Bibarel", 12, 17), ("Kricketot", 0, -1), ("Kricketune", 0, -1), ("Shinx", 3, -1), ("Luxio", 3, -1), ("Luxray", 3, -1), ("Budew", 9, 13), ("Roserade", 9, 13), ("Cranidos", 15, -1), ("Rampardos", 15, -1), ("Shieldon", 15, 16), ("Bastiodon", 15, 16), ("Burmy", 0, -1), ("Wormadam", 0, 9), ("Mothim", 0, 7), ("Combee", 0, 7), ("Vespiquen", 0, 7), ("Pachirisu", 3, -1), ("Buizel", 17, -1), ("Floatzel", 17, -1), ("Cherubi", 9, -1), ("Cherrim", 9, -1), ("Shellos", 17, -1), ("Gastrodon", 17, 10), ("Ambipom", 12, -1), ("Drifloon", 8, 7), ("Drifblim", 8, 7), ("Buneary", 12, -1), ("Lopunny", 12, -1), ("Mismagius", 8, -1), ("Honchkrow", 1, 7), ("Glameow", 12, -1), ("Purugly", 12, -1), ("Chingling", 14, -1), ("Stunky", 13, 1), ("Skuntank", 13, 1), ("Bronzor", 16, 14), ("Bronzong", 16, 14), ("Bonsly", 15, -1), ("Mime Jr.", 14, 4), ("Happiny", 12, -1), ("Chatot", 12, 7), ("Spiritomb", 8, 1), ("Gible", 2, 10), ("Gabite", 2, 10), ("Garchomp", 2, 10), ("Munchlax", 12, -1), ("Riolu", 5, -1), ("Lucario", 5, 16), ("Hippopotas", 10, -1), ("Hippowdon", 10, -1), ("Skorupi", 13, 0), ("Drapion", 13, 1), ("Croagunk", 13, 5), ("Toxicroak", 13, 5), ("Carnivine", 9, -1), ("Finneon", 17, -1), ("Lumineon", 17, -1), ("Mantyke", 17, 7), ("Snover", 9, 11), ("Abomasnow", 9, 11), ("Weavile", 1, 11), ("Magnezone", 3, 16), ("Lickilicky", 12, -1), ("Rhyperior", 10, 15), ("Tangrowth", 9, -1), ("Electivire", 3, -1), ("Magmortar", 6, -1), ("Togekiss", 4, 7), ("Yanmega", 0, 7), ("Leafeon", 9, -1), ("Glaceon", 11, -1), ("Gliscor", 10, 7), ("Mamoswine", 11, 10), ("Porygon-Z", 12, -1), ("Gallade", 14, 5), ("Probopass", 15, 16), ("Dusknoir", 8, -1), ("Froslass", 11, 8), ("Rotom", 3, 8), ("Uxie", 14, -1), ("Mesprit", 14, -1), ("Azelf", 14, -1), ("Dialga", 16, 2), ("Palkia", 17, 2), ("Heatran", 6, 16), ("Regigigas", 12, -1), ("Giratina", 8, 2), ("Cresselia", 14, -1), ("Phione", 17, -1), ("Manaphy", 17, -1), ("Darkrai", 1, -1), ("Shaymin", 9, -1), ("Arceus", 12, -1), ("Victini", 14, 6), ("Snivy", 9, -1), ("Servine", 9, -1), ("Serperior", 9, -1), ("Tepig", 6, -1), ("Pignite", 6, 5), ("Emboar", 6, 5), ("Oshawott", 17, -1), ("Dewott", 17, -1), ("Samurott", 17, -1), ("Patrat", 12, -1), ("Watchog", 12, -1), ("Lillipup", 12, -1), ("Herdier", 12, -1), ("Stoutland", 12, -1), ("Purrloin", 1, -1), ("Liepard", 1, -1), ("Pansage", 9, -1), ("Simisage", 9, -1), ("Pansear", 6, -1), ("Simisear", 6, -1), ("Panpour", 17, -1), ("Simipour", 17, -1), ("Munna", 14, -1), ("Musharna", 14, -1), ("Pidove", 12, 7), ("Tranquill", 12, 7), ("Unfezant", 12, 7), ("Blitzle", 3, -1), ("Zebstrika", 3, -1), ("Roggenrola", 15, -1), ("Boldore", 15, -1), ("Gigalith", 15, -1), ("Woobat", 14, 7), ("Swoobat", 14, 7), ("Drilbur", 10, -1), ("Excadrill", 10, 16), ("Audino", 12, -1), ("Timburr", 5, -1), ("Gurdurr", 5, -1), ("Conkeldurr", 5, -1), ("Tympole", 17, -1), ("Palpitoad", 17, 10), ("Seismitoad", 17, 10), ("Throh", 5, -1), ("Sawk", 5, -1), ("Sewaddle", 0, 9), ("Swadloon", 0, 9), ("Leavanny", 0, 9), ("Venipede", 0, 13), ("Whirlipede", 0, 13), ("Scolipede", 0, 13), ("Cottonee", 9, 4), ("Whimsicott", 9, 4), ("Petilil", 9, -1), ("Lilligant", 9, -1), ("Basculin", 17, -1), ("Sandile", 10, 1), ("Krokorok", 10, 1), ("Krookodile", 10, 1), ("Darumaka", 6, -1), ("Darmanitan", 6, -1), ("Maractus", 9, -1), ("Dwebble", 0, 15), ("Crustle", 0, 15), ("Scraggy", 1, 5), ("Scrafty", 1, 5), ("Sigilyph", 14, 7), ("Yamask", 8, -1), ("Cofagrigus", 8, -1), ("Tirtouga", 17, 15), ("Carracosta", 17, 15), ("Archen", 15, 7), ("Archeops", 15, 7), ("Trubbish", 13, -1), ("Garbodor", 13, -1), ("Zorua", 1, -1), ("Zoroark", 1, -1), ("Minccino", 12, -1), ("Cinccino", 12, -1), ("Gothita", 14, -1), ("Gothorita", 14, -1), ("Gothitelle", 14, -1), ("Solosis", 14, -1), ("Duosion", 14, -1), ("Reuniclus", 14, -1), ("Ducklett", 17, 7), ("Swanna", 17, 7), ("Vanillite", 11, -1), ("Vanillish", 11, -1), ("Vanilluxe", 11, -1), ("Deerling", 12, 9), ("Sawsbuck", 12, 9), ("Emolga", 3, 7), ("Karrablast", 0, -1), ("Escavalier", 0, 16), ("Foongus", 9, 13), ("Amoonguss", 9, 13), ("Frillish", 17, 8), ("Jellicent", 17, 8), ("Alomomola", 17, -1), ("Joltik", 0, 3), ("Galvantula", 0, 3), ("Ferroseed", 9, 16), ("Ferrothorn", 9, 16), ("Klink", 16, -1), ("Klang", 16, -1), ("Klinklang", 16, -1), ("Tynamo", 3, -1), ("Eelektrik", 3, -1), ("Eelektross", 3, -1), ("Elgyem", 14, -1), ("Beheeyem", 14, -1), ("Litwick", 8, 6), ("Lampent", 8, 6), ("Chandelure", 8, 6), ("Axew", 2, -1), ("Fraxure", 2, -1), ("Haxorus", 2, -1), ("Cubchoo", 11, -1), ("Beartic", 11, -1), ("Cryogonal", 11, -1), ("Shelmet", 0, -1), ("Accelgor", 0, -1), ("Stunfisk", 3, 10), ("Mienfoo", 5, -1), ("Mienshao", 5, -1), ("Druddigon", 2, -1), ("Golett", 10, 8), ("Golurk", 10, 8), ("Pawniard", 1, 16), ("Bisharp", 1, 16), ("Bouffalant", 12, -1), ("Rufflet", 12, 7), ("Braviary", 12, 7), ("Vullaby", 1, 7), ("Mandibuzz", 1, 7), ("Heatmor", 6, -1), ("Durant", 0, 16), ("Deino", 1, 2), ("Zweilous", 1, 2), ("Hydreigon", 1, 2), ("Larvesta", 0, 6), ("Volcarona", 0, 6), ("Cobalion", 16, 5), ("Terrakion", 15, 5), ("Virizion", 9, 5), ("Tornadus", 7, -1), ("Thundurus", 3, 7), ("Reshiram", 2, 6), ("Zekrom", 2, 3), ("Landorus", 10, 7), ("Kyurem", 2, 11), ("Keldeo", 17, 5), ("Meloetta", 12, 14), ("Genesect", 0, 16), ("Chespin", 9, -1), ("Quilladin", 9, -1), ("Chesnaught", 9, 5), ("Fennekin", 6, -1), ("Braixen", 6, -1), ("Delphox", 6, 14), ("Froakie", 17, -1), ("Frogadier", 17, -1), ("Greninja", 17, 1), ("Bunnelby", 12, -1), ("Diggersby", 12, 10), ("Fletchling", 12, 7), ("Fletchinder", 6, 7), ("Talonflame", 6, 7), ("Scatterbug", 0, -1), ("Spewpa", 0, -1), ("Vivillon", 0, 7), ("Litleo", 6, 12), ("Pyroar", 6, 12), ("Flabébé", 4, -1), ("Floette", 4, -1), ("Florges", 4, -1), ("Skiddo", 9, -1), ("Gogoat", 9, -1), ("Pancham", 5, -1), ("Pangoro", 5, 1), ("Furfrou", 12, -1), ("Espurr", 14, -1), ("Meowstic", 14, -1), ("Honedge", 16, 8), ("Doublade", 16, 8), ("Aegislash", 16, 8), ("Spritzee", 4, -1), ("Aromatisse", 4, -1), ("Swirlix", 4, -1), ("Slurpuff", 4, -1), ("Inkay", 1, 14), ("Malamar", 1, 14), ("Binacle", 15, 17), ("Barbaracle", 15, 17), ("Skrelp", 13, 17), ("Dragalge", 13, 2), ("Clauncher", 17, -1), ("Clawitzer", 17, -1), ("Helioptile", 3, 12), ("Heliolisk", 3, 12), ("Tyrunt", 15, 2), ("Tyrantrum", 15, 2), ("Amaura", 15, 11), ("Aurorus", 15, 11), ("Sylveon", 4, -1), ("Hawlucha", 5, 7), ("Dedenne", 3, 4), ("Carbink", 15, 4), ("Goomy", 2, -1), ("Sliggoo", 2, -1), ("Goodra", 2, -1), ("Klefki", 16, 4), ("Phantump", 8, 9), ("Trevenant", 8, 9), ("Pumpkaboo", 8, 9), ("Gourgeist", 8, 9), ("Bergmite", 11, -1), ("Avalugg", 11, -1), ("Noibat", 7, 2), ("Noivern", 7, 2), ("Xerneas", 4, -1), ("Yveltal", 1, 7), ("Zygarde", 2, 10)
    ];

    let (name, idx1, idx2) = POKEMONS[buf.trim().parse::<usize>().unwrap() - 1];

    if idx2 == -1 {
        println!("{name}\n{}", TYPES[idx1]);
    } else {
        println!("{name}\n{} {}", TYPES[idx1], TYPES[idx2 as usize]);
    }
}