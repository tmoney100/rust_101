trait Movable {
  fn move_guy(&mut self, x: i64, y: i64);
}

struct Player {
  x: i64,
  y: i64,
}

impl Movable for Player {
  fn move_guy(&mut self, x: i64, y: i64) {
      self.x += x;
      self.y += y;
  }
}

fn main() {
  let mut player = Player {
      x: 100,
      y: 0,
  };

  player.move_guy(10, 10);
  println!("Player's position: x = {}, y = {}", player.x, player.y);  
}
