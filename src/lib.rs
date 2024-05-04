// Define the game configuration using the turbo::cfg! macro
turbo::cfg! {r#"
    name = "angry dolls"
    version = "1.0.0"
    author = "Turbo"
    description = "angry-dolls!"
    [settings]
    resolution = [256, 144]
"#}

// Define the game state initialization using the turbo::init! macro
turbo::init! {
    struct GameState {
        frame: u32,
        npc_blue_x: f32,
        npc_blue_y: f32,
        npc_spex_x: f32, // Add NPC "npc_spex" x position
        npc_spex_y: f32, // Add NPC "npc_spex" y position
        lives_1: u32,
        lives_2: u32,
        // Add bullets for NPC "npc_bluey"
        bullets_1: Vec<struct Bullet {
            x: f32,
            y: f32,
            vel_x: f32, // Velocity in the x-axis direction
            vel_y: f32, // Velocity in the y-axis direction
            size: f32,
        }>,
       bullets_2: Vec<struct Bullets {
           x: f32,
           y: f32,
           vel_x: f32, // Velocity in the x-axis direction
           vel_y: f32, // Velocity in the y-axis direction
           size: f32,

       }>,
    } = {
        Self {
            frame: 0,
            npc_blue_x: 20.0,
            npc_blue_y: 120.0,
            npc_spex_x: 190.0, // Initial position for NPC "npc_spex"
            npc_spex_y: 120.0,  // Initial position for NPC "npc_spex"
            lives_1: 10,
            lives_2: 10,
            bullets_1: vec![],
            bullets_2: vec![],
        }
    }
}

// Implement the game loop using the turbo::go! macro
turbo::go! {
    // Load the game state
    let mut state = GameState::load();



  // Handle user input
let game_input = gamepad(0);
let game_input1 = gamepad(1);



    // Move the cat based on keyboard arrow keys
    if game_input.left.pressed() {
        state.npc_blue_x -= 2.;
    }
    if game_input.right.pressed() {
        state.npc_blue_x += 2.;
    }



        // Move the cat based on keyboard arrow keys
        if game_input1.left.pressed() {
            state.npc_spex_x-= 2.;
        }
        if game_input1.right.pressed() {
            state.npc_spex_x += 2.;
        }


        // Generate bullets for NPC "npc_bluey" only when 'A' key is pressed and shooting is allowed
        if game_input.a.pressed() && state.lives_1 > 0 && state.lives_2 > 0 {
            if rand() % 10 == 0 {
                let bullet = Bullet {
                    x: state.npc_blue_x,
                    y: state.npc_blue_y,
                    vel_x: 5.0,
                    vel_y: 0.0,
                    size: 5.0,
                };
                state.bullets_1.push(bullet);
            }
        }

   // Update bullet positions for npc_bluey
   for bullet in &mut state.bullets_1 {
       bullet.x += bullet.vel_x; // Move the bullet horizontally (adjust direction and velocity as needed)
   }
   // Handle collisions for bullets shot by npc_bluey
   let mut bullets_to_remove_1 = Vec::new();
   for bullet in &mut state.bullets_1 {
       // Check if bullet hits npc_spex
       if bullet.x >= state.npc_spex_x && bullet.y >= state.npc_spex_y {
           // If collision, decrement lives of npc_spex and mark the bullet for removal
           state.lives_2 -= 1;
           bullets_to_remove_1.push(bullet.clone());
       }
   }

   // Remove bullets that collided with npc_spex
   state.bullets_1.retain(|bullet| !bullets_to_remove_1.contains(bullet));




   // Generate bullets for NPC "npc_spex" only when 'A' key is pressed and shooting is allowed
   if game_input1.a.pressed() && state.lives_1 > 0 && state.lives_2 > 0  {
       if rand() % 10 == 0 {
           let bullet = Bullets {
               x: state.npc_spex_x,
               y: state.npc_spex_y,
               vel_x: -5.0,
               vel_y: 0.0,
               size: 5.0,
           };
           state.bullets_2.push(bullet);
       }
   }
      // Update bullet positions for npc_spex
      for bullet in &mut state.bullets_2 {
          bullet.x += bullet.vel_x; // Move the bullet horizontally (adjust direction and velocity as needed)
      }
      // Handle collisions for bullets shot by npc_spex
      let mut bullets_to_remove_2 = Vec::new();
      for bullet in &mut state.bullets_2 {
          // Check if bullet hits npc_bluey
          if bullet.x <= state.npc_blue_x && bullet.y <= state.npc_blue_y {
              // If collision, decrement lives of npc_bluey and mark the bullet for removal
              state.lives_1 -= 1;
              bullets_to_remove_2.push(bullet.clone());
          }
      }

      // Remove bullets that collided with npc_bluey
      state.bullets_2.retain(|bullet| !bullets_to_remove_2.contains(bullet));



   // Set the background color
   clear(0x00ffffff);

   // Draw a tiled background of moving sprites
   let frame = (state.frame as i32) / 2;
   let cloud_width = 32; // Width of each cloud sprite
   let gap = 24; // Gap between each cloud

   for col in 0..9 {
       let x = col * (cloud_width + gap); // Include gap between each cloud
       let y = 0; // Set y position to keep clouds fixed at the top of the screen
       let x = ((x + frame) % (272 + 16)) - 32; // Calculate x position for cloud's horizontal movement
       sprite!("cloud_02", x = x, y = y);
   }
   // Draw other sprites (omitted for brevity)
   sprite!("bg_mountains", x = (0) as i32, y = (70) as i32, fps = fps::FAST);
   sprite!("trees_group", x = (0) as i32, y = (90) as i32, fps = fps::FAST);
   sprite!("npc_bluey", x = (state.npc_blue_x - 16.0) as i32, y = (state.npc_blue_y - 16.0) as i32, fps = fps::FAST);
   sprite!("npc_spex", x = (state.npc_spex_x - 16.0) as i32, y = (state.npc_spex_y - 16.0) as i32, fps = fps::FAST);

   // Render bullets for NPC "npc_bluey"
   for bullet in &state.bullets_1 {
       circ!(x = bullet.x as i32, y = bullet.y as i32, d = bullet.size as u32, color = 0x00ff00ff); // Render the bullets
   }

   // Render bullets for NPC "npc_spex"
   for bullet in &state.bullets_2 {
       circ!(x = bullet.x as i32, y = bullet.y as i32, d = bullet.size as u32, color = 0x00ff00ff); // Render the bullets
   }


   // Draw the lives
   text!(&format!("Lives: {}", state.lives_1), x = 10, y = 24, font = Font::L, color = 0xffffffff);
   // Draw the lives
   text!(&format!("Lives: {}", state.lives_2), x =  175, y = 24, font = Font::L, color = 0xffffffff);

   // Define a boolean flag to indicate game restart
   let mut restart_game = false;

   // Check if any player has zero lives
   if state.lives_1 == 0 {
       text!("ANGRY DAD Wins!", x = 80, y = 60, font = Font::L, color = 0xff0000ff);
       text!("Press 'space' to Restart", x = 60, y = 80, font = Font::L, color = 0xff0000ff);
       if game_input.start.pressed() {
           restart_game = true;
       }
   }
   if state.lives_2 == 0 {
       text!("ANGRY SON Wins!", x = 80, y = 60, font = Font::L, color = 0xff0000ff);
       text!("Press 'Space' to Restart", x = 60, y = 80, font = Font::L, color = 0xff0000ff);
       if game_input.start.pressed() {
           restart_game = true;
       }
   }

   // Restart the game if the restart_game flag is true
   if restart_game {
       state = GameState {
           frame: 0,
           npc_blue_x: 20.0,
           npc_blue_y: 120.0,
           npc_spex_x: 190.0,
           npc_spex_y: 120.0,
           lives_1: 10,
           lives_2: 10,
           bullets_1: vec![],
           bullets_2: vec![],
       };
   };



   // Save game state for the next frame
   state.frame += 1;
   state.save();
}
