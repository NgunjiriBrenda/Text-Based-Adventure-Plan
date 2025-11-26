// Import necessary Rust standard library components
use std::collections::HashMap;
use std::io;
use std::thread;
use std::time::Duration;

// Define possible directions the player can move
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South, 
    East,
    West,
}

// Define what a Room contains
struct Room {
    name: String,
    description: String,
    ascii_art: &'static str,  // ASCII art for visual representation
    exits: HashMap<Direction, String>,
}

// Implement methods for the Room struct
impl Room {
    fn new(name: &str, description: &str, ascii_art: &'static str) -> Self {
        Room {
            name: name.to_string(),
            description: description.to_string(),
            ascii_art,
            exits: HashMap::new(),
        }
    }
    
    fn add_exit(mut self, direction: Direction, room_name: &str) -> Self {
        self.exits.insert(direction, room_name.to_string());
        self
    }
}

// Clear screen function
fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

// Main function - entry point of our program
fn main() {
    clear_screen();
    
    // Show animated title
    show_animated_title();
    
    // ASCII Art for different rooms
    let dungeon_art = r#"
    ┌─────────────────┐
    │    🚪░░░░░░░    │
    │   ░░░▒▒▒▒▒░░░   │
    │  ░░▒▒     ▒▒░░  │
    │ ░░▒▒  🕳️   ▒▒░░ │
    │ ░░▒▒       ▒▒░░ │
    │  ░░▒▒▒▒▒▒▒▒░░   │
    │   ░░░░░░░░░░    │
    └─────────────────┘
    DUNGEON CELL
    "#;
    
    let hallway_art = r#"
    ┌─────────────────┐
    │ 🕯️             🕯️ │
    │                 │
    │    ────────     │
    │                 │
    │ 🕯️             🕯️ │
    │                 │
    │    ────────     │
    └─────────────────┘
    STONE HALLWAY
    "#;
    
    let throne_art = r#"
    ┌─────────────────┐
    │      ___        │
    │     /___\       │
    │    🐉|_|🐉      │
    │    💎💰💎     │
    │   📦TREASURE📦  │
    │                 │
    │     🪑THRONE🪑   │
    └─────────────────┘
    THRONE ROOM
    "#;

    // Create game world with ASCII art
    let dungeon = Room::new(
        "Dungeon Cell",
        "A cold, dark prison cell. Stone walls surround you.\nThere's a rusty door to the NORTH.",
        dungeon_art
    ).add_exit(Direction::North, "Hallway");

    let hallway = Room::new(
        "Hallway", 
        "A torch-lit hallway with ancient tapestries.\nExits lead SOUTH and EAST.",
        hallway_art
    ).add_exit(Direction::South, "Dungeon Cell")
     .add_exit(Direction::East, "Throne Room");

    let throne_room = Room::new(
        "Throne Room",
        "A magnificent room with dragon-carved throne!\nGolden treasures sparkle everywhere!\nYou found the dragon's treasure hoard!",
        throne_art
    ).add_exit(Direction::West, "Hallway");

    // Store all rooms
    let mut rooms = HashMap::new();
    rooms.insert(dungeon.name.clone(), dungeon);
    rooms.insert(hallway.name.clone(), hallway);
    rooms.insert(throne_room.name.clone(), throne_room);

    let mut current_room_name = "Dungeon Cell".to_string();
    
    // Start game
    game_loop(&mut current_room_name, &rooms);
}

fn show_animated_title() {
    let frames = [
        r#"
    🐉 🏰 🐉 🏰 🐉 🏰 🐉 🏰 🐉 
          DRAGON'S ESCAPE
    🐉 🏰 🐉 🏰 🐉 🏰 🐉 🏰 🐉 
        "#,
        r#"
    🏰 🐉 🏰 🐉 🏰 🐉 🏰 🐉 🏰 
          DRAGON'S ESCAPE  
    🏰 🐉 🏰 🐉 🏰 🐉 🏰 🐉 🏰 
        "#
    ];
    
    for _ in 0..3 {
        for frame in &frames {
            clear_screen();
            println!("{}", frame);
            thread::sleep(Duration::from_millis(500));
        }
    }
    
    println!("You are a brave adventurer trapped in a dragon's castle!");
    println!("Explore rooms, find treasures, and escape to freedom!\n");
    println!("Press ENTER to begin your adventure...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
}

fn game_loop(current_room_name: &mut String, rooms: &HashMap<String, Room>) {
    let mut visited_throne_room = false;
    
    loop {
        clear_screen();
        
        // Display current room with visual elements
        if let Some(current_room) = rooms.get(current_room_name) {
            display_room(current_room);
            
            // Special message for first time in throne room
            if current_room.name == "Throne Room" && !visited_throne_room {
                println!();
                show_treasure_animation();
                visited_throne_room = true;
            }
        }
        
        // Show compass for navigation
        show_compass(current_room_name, rooms);
        
        // Get player input
        println!();
        println!("┌────────────────────────────────────┐");
        println!("│           YOUR COMMAND             │");
        println!("└────────────────────────────────────┘");
        print!("> ");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_lowercase();
        
        // Process commands
        match input.as_str() {
            "quit" | "exit" => {
                show_goodbye();
                break;
            }
            
            "look" => {
                println!("You examine your surroundings carefully...");
                thread::sleep(Duration::from_secs(1));
            }
            
            "help" => {
                show_help();
                wait_for_enter();
            }
            
            "map" => {
                show_map();
                wait_for_enter();
            }
            
            _ if input.starts_with("go ") => {
                handle_movement(&input, current_room_name, rooms);
                thread::sleep(Duration::from_secs(1));
            }
            
            "" => {
                // Do nothing, just refresh
            }
            
            _ => {
                println!("❌ Unknown command: '{}'", input);
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

fn display_room(room: &Room) {
    println!("┌────────────────────────────────────┐");
    println!("│{:^36}│", room.name);
    println!("└────────────────────────────────────┘");
    println!("{}", room.ascii_art);
    println!("┌────────────────────────────────────┐");
    println!("│           ROOM DESCRIPTION         │");
    println!("└────────────────────────────────────┘");
    println!("{}", room.description);
}

fn show_compass(current_room_name: &str, rooms: &HashMap<String, Room>) {
    if let Some(current_room) = rooms.get(current_room_name) {
        println!();
        println!("┌────────────────────────────────────┐");
        println!("│              COMPASS               │");
        println!("├────────────────────────────────────┤");
        
        let north = if current_room.exits.contains_key(&Direction::North) { "🔼 NORTH " } else { "        " };
        let west = if current_room.exits.contains_key(&Direction::West) { "◀ WEST " } else { "       " };
        let east = if current_room.exits.contains_key(&Direction::East) { "EAST ▶" } else { "      " };
        let south = if current_room.exits.contains_key(&Direction::South) { "🔽 SOUTH " } else { "        " };
        
        println!("│{:^36}│", north);
        println!("│{:^36}│", format!("{}   {}", west, east));
        println!("│{:^36}│", south);
        println!("└────────────────────────────────────┘");
    }
}

fn show_treasure_animation() {
    let treasures = ["💎", "💰", "👑", "💍", "🏆", "🔮"];
    
    for _ in 0..2 {
        for treasure in &treasures {
            clear_screen();
            println!("┌────────────────────────────────────┐");
            println!("│            TREASURE FOUND!         │");
            println!("└────────────────────────────────────┘");
            println!();
            println!("{:^36}", treasure);
            println!();
            println!("🎉 YOU FOUND THE DRAGON'S HOARD! 🎉");
            println!("The treasure glitters before you!");
            thread::sleep(Duration::from_millis(200));
        }
    }
}

fn handle_movement(input: &str, current_room_name: &mut String, rooms: &HashMap<String, Room>) {
    let direction_str = input.trim_start_matches("go ").trim();
    
    let direction = match direction_str {
        "north" => Some(Direction::North),
        "south" => Some(Direction::South),
        "east" => Some(Direction::East),
        "west" => Some(Direction::West),
        _ => {
            println!("❌ Unknown direction: '{}'", direction_str);
            None
        }
    };
    
    if let Some(dir) = direction {
        if let Some(current_room) = rooms.get(current_room_name) {
            if let Some(next_room_name) = current_room.exits.get(&dir) {
                // Show movement animation
                show_movement_animation(direction_str);
                *current_room_name = next_room_name.clone();
            } else {
                println!("❌ You can't go that way!");
            }
        }
    }
}

fn show_movement_animation(direction: &str) {
    let arrows = match direction {
        "north" => "🔼",
        "south" => "🔽", 
        "east" => "▶",
        "west" => "◀",
        _ => "🚶"
    };
    
    for i in 0..3 {
        clear_screen();
        println!("┌────────────────────────────────────┐");
        println!("│              MOVING...            │");
        println!("└────────────────────────────────────┘");
        println!();
        println!("{:^36}", ".".repeat(i + 1));
        println!("{:^36}", arrows);
        println!("{:^36}", "Moving ".to_string() + &direction.to_uppercase());
        thread::sleep(Duration::from_millis(300));
    }
}

fn show_help() {
    clear_screen();
    println!("┌────────────────────────────────────┐");
    println!("│              HELP MENU             │");
    println!("├────────────────────────────────────┤");
    println!("│ 🎮 COMMANDS:                       │");
    println!("│   go north/south/east/west         │");
    println!("│   look - Examine room              │");
    println!("│   map - Show game map              │");
    println!("│   help - This menu                 │");
    println!("│   quit - Exit game                 │");
    println!("├────────────────────────────────────┤");
    println!("│ 🎯 GOAL:                           │");
    println!("│   Find the treasure in the         │");
    println!("│   Throne Room!                     │");
    println!("├────────────────────────────────────┤");
    println!("│ 💡 TIPS:                           │");
    println!("│   • Start in Dungeon Cell          │");
    println!("│   • Go North to Hallway            │");
    println!("│   • Go East to Throne Room         │");
    println!("│   • Find the treasure!             │");
    println!("└────────────────────────────────────┘");
}

fn show_map() {
    clear_screen();
    println!("┌────────────────────────────────────┐");
    println!("│            CASTLE MAP              │");
    println!("├────────────────────────────────────┤");
    println!("│                                    │");
    println!("│        🏰 THRONE ROOM 🏰          │");
    println!("│              │                    │");
    println!("│              │                    │");
    println!("│ WEST ← HALLWAY → EAST             │");
    println!("│              │                    │");
    println!("│              │                    │");
    println!("│           DUNGEON 🕳️              │");
    println!("│                                    │");
    println!("└────────────────────────────────────┘");
    println!("You are exploring a dragon's castle!");
    println!("Find your way to the treasure!");
}

fn show_goodbye() {
    clear_screen();
    println!("┌────────────────────────────────────┐");
    println!("│            FAREWELL!               │");
    println!("├────────────────────────────────────┤");
    println!("│                                    │");
    println!("│      Thanks for playing!           │");
    println!("│    🐉 Dragon's Escape 🐉         │");
    println!("│                                    │");
    println!("│   Come back for more adventures!   │");
    println!("│                                    │");
    println!("└────────────────────────────────────┘");
}

fn wait_for_enter() {
    println!();
    println!("Press ENTER to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
}