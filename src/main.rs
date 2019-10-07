#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Creature {
	Empty = 0,
	Grass = 1,
	Sheep = 2,
	Wolf = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Position {
	column: u32,
	row: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Life {
	Dead = 0,
	Alive = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Inside {
	id:u32,
	creature: Creature,
	position: Position,
	stamina:u8,
	status: Life,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
	id:u32,
	creature: Creature,
	position: Position,
	stamina:u8,
	status: Life,
	inside: Option<Inside>,
	direction: Direction
}

#[derive(Debug)]
pub struct Organizm {
	position: Position,
	creature: Creature,
	stamina:u8,
	status: Life
}

pub struct Universe {
	height: u32,
	width: u32,
	cells: Vec<Cell>,
	creature_count: u32
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
	Stand,
	North,
	NorthEast,
	East,
	SouthEast,
	South,
	SouthWest,
	West,
	NorthWest,
}

impl Direction {
	fn get_direction(&self) -> (i32,i32){
		match self {
			Direction::Stand => (0,0),
			Direction::North => (0,-1),
			Direction::NorthEast => (1,-1),
			Direction::East => (1,0),
			Direction::SouthEast => (1,1),
			Direction::South => (0,1),
			Direction::SouthWest => (-1,1),
			Direction::West => (-1,0),
			Direction::NorthWest => (-1,-1),
		}
	}
}

fn get_coordinate(width: u32,index: usize) -> Position{
	let index = index as u32;
	let column = (index)% width as u32;
	let row = (index) / width as u32;
	Position{row,column}
}

use std::cmp::Ordering;
impl Universe {
	fn get_index(&self, row: u32, column: u32) -> usize {
		(row * self.width + column) as usize
	}

	fn get_position_with_id(&self,cell_id: u32)->Option<Position>{
		for each_cell in self.cells.iter() {
			if each_cell.id == cell_id{
					return Some(each_cell.position);
			}
		}
		return None;
	}

	fn get_index_with_id(&self,cell_id: u32) -> Option<usize>{
		match self.get_position_with_id(cell_id) {
			Some(position)=> {
				return Some(self.get_index(position.row, position.column));
			},
			None => None
		}
	}

	pub fn get_food_count(&self,column:u32,row:u32){
		let mut count = 0;
		let object_index = self.get_index(row, column);
		let food_type = match (self.cells[object_index].creature,self.cells[object_index].status) {
			(Creature::Empty, _) => 0,
			(Creature::Grass, _) => 0,
			(Creature::Sheep, Life::Alive) => 1,
			(Creature::Wolf, Life::Alive) => 2,
			(_, Life::Dead) => 0,
		};
		for delta_row in [self.height-1,0,1].iter().cloned() {
			for delta_col in [self.width-1,0,1].iter().cloned() {
				if delta_col == 0 && delta_row == 0{
					continue;
				}
				let neighbor_row = (row+delta_row) % self.height;
				let neighbor_col = (row+delta_col) % self.width;
				let neighbor_index = self.get_index(neighbor_row, neighbor_col);
				let creature = self.cells[neighbor_index].creature as u8;
				count += match creature.cmp(&food_type){
					Ordering::Equal => 1,
					_ => 0
				};
			}
		}
		println!("{}",count);
	}

	pub fn new(height: u32, width: u32) -> Universe{
		let cells:Vec<Cell> = (0..width*height).map(|i|{
			let index = i.clone() as usize;
			let position = get_coordinate(width, index);
			if i%2 == 0 || i%7 == 0{
				let new_object = Cell{
					id: i,
					position,
					creature: Creature::Grass,
					stamina: 40,
					status: Life::Dead,
					inside: None,
					direction: Direction::Stand
				};
				(new_object)
			}else{
				let new_object = Cell{
					id: i,
					position,
					creature: Creature::Empty,
					stamina: 0,
					status: Life::Dead,
					inside: None,
					direction: Direction::Stand
				};
				(new_object)
			}
		}).collect();
		let creature_count = *&cells.len() as u32;
		Universe {
			width,
			height,
			cells,
			creature_count
		}
	}

	pub fn set(&mut self,new_creature: Organizm){
		let mut next = self.cells.clone();
		let cell_index = self.get_index(new_creature.position.row,new_creature.position.column);
		let position = get_coordinate(self.width, cell_index);
		next[cell_index] = Cell{
			id: self.creature_count+1,
			position,
			creature: new_creature.creature,
			stamina: new_creature.stamina,
			status: new_creature.status,
			inside: Some(Inside{
				id: cell_index as u32,
				position,
				creature: next[cell_index].creature,
				stamina: next[cell_index].stamina,
				status: next[cell_index].status,}),
			direction: Direction::Stand
		};
		// println!("{:?}",next[cell_index]);
		self.cells = next;
		self.creature_count+=1;
	}
}

use std::fmt;
impl fmt::Display for Universe {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for line in self.cells.as_slice().chunks(self.width as usize) {
			for &cell in line {
				// let symbol = if cell.creature == Creature::Empty { '◻' } else { '◼' };
				let symbol =
				if cell.creature == Creature::Empty { '*' }
				else if cell.creature == Creature::Grass { '#' }
				else if cell.creature == Creature::Sheep { 'S' }
				else if cell.creature == Creature::Wolf { 'W' }
				else { ' ' };
				write!(f, "{}", symbol)?;
			}
			write!(f, "\n")?;
		}
		Ok(())
	}
}

impl Cell {
	fn kill_cell(&mut self){
		*self = match self.inside {
			Some(inside_cell) => {
				Cell{
					id: inside_cell.id,
					creature: inside_cell.creature,
					position: self.position,
					stamina: inside_cell.stamina + self.stamina,
					status: Life::Dead,
					inside: None,
					direction: Direction::Stand
				}
			}
			None => {
				Cell{
					id: self.id,
					creature: Creature::Grass,
					position: self.position,
					stamina: self.stamina,
					status: Life::Dead,
					inside: None,
					direction: Direction::Stand
				}
			}
		};
	}


	fn check_for_kill(&mut self){
		if self.stamina == 0 {
			self.kill_cell();
		}
	}

	fn feed(&mut self){
		match  self.inside {
			Some(mut inside) =>{
				if inside.stamina > 0 {
					match inside.stamina.cmp(&5){
						Ordering::Less => {
							self.stamina += 1;
							inside.stamina -= 1;
							self.inside = Some(inside);
						},
						Ordering::Greater => {
							self.stamina += 6;
							inside.stamina -= 6;
							self.inside = Some(inside);
						},
						Ordering::Equal => {
							self.stamina += 5;
							inside.stamina -= 5;
							self.inside = Some(inside);
						}
					};
				}
			},
			None=>()
		}
	}

	fn starve(&mut self){
		self.check_for_kill();
		self.stamina -= 1;
		self.check_for_kill();
	}
}

// Движение
impl Universe{
	fn get_new_index_with_direction(&self, standing_index: usize, x:(i32,i32)) -> usize{
		let i32_height = self.height as i32;
		let i32_width = self.width as i32;

		let cell_positions = self.cells[standing_index].position;
		let cell_column = cell_positions.column as i32;
		let cell_row = cell_positions.row as i32;

		let delta_col = if x.0 == -1 {i32_width-1}	else {x.0};
		let delta_row = if x.1 == -1 {i32_height-1}	else {x.1};

		let new_col = ((cell_column + delta_col) % i32_width) as u32;
		let new_row = ((cell_row + delta_row) % i32_height) as u32;
		self.get_index(new_row, new_col)
	}

	fn make_movement(&mut self){
		let mut next = self.cells.clone();
		for cell in self.cells.iter(){
			if cell.status == Life::Alive && cell.stamina > 1{
				let cell_index = self.get_index_with_id(cell.id);
				match cell_index {
					Some(index) => {
						let have_been_eaten_cell = match cell.inside {
							Some(e_cell) => Cell{
								id: e_cell.id,
								creature: e_cell.creature,
								position: e_cell.position,
								stamina: e_cell.stamina,
								status: e_cell.status,
								inside: None,
								direction: Direction::Stand
							},
							None => Cell{
								id: index as u32,
								creature: Creature::Empty,
								position: cell.position,
								stamina: 0,
								status: Life::Dead,
								inside: None,
								direction: Direction::Stand
							}
						};
						let direction = Direction::North.get_direction();
						let new_index: usize = self.get_new_index_with_direction(index, direction);
						// let new_index = (index+1) % self.cells.len();
						let will_eatten_cell = Inside{
							id: self.cells[new_index].id,
							creature: self.cells[new_index].creature,
							position: self.cells[new_index].position,
							stamina: self.cells[new_index].stamina,
							status: self.cells[new_index].status,
						};
						let creature_in_new_place = Cell{
							id: self.cells[index].id,
							creature: self.cells[index].creature,
							position: self.cells[new_index].position,
							stamina: self.cells[index].stamina - 1,
							status: self.cells[index].status,
							inside: Some(will_eatten_cell),
							direction: Direction::Stand
						};
						next[index] = have_been_eaten_cell;
						next[new_index] = creature_in_new_place;
						()
					},
					None => (),
				}
			}
		}
		self.cells = next;
	}

	pub fn tick(&mut self){
		let mut next = self.cells.clone();
		for cell in self.cells.iter() {
			if cell.status == Life::Alive{
				match self.get_index_with_id(cell.id){
					Some(cell_index) => {
						next[cell_index].starve();
						next[cell_index].feed();
					}
					None => ()
				}
			}
		}
		self.cells = next;
		self.make_movement();
	}
}


fn main() {
	let mut universe_1 = Universe::new(5,5);
	let sheep_1 = Organizm{
		position: Position{column:0,row:0},
		creature: Creature::Sheep,
		stamina: 10,
		status: Life::Alive
	};
	universe_1.set(sheep_1);
	universe_1.tick();
	universe_1.tick();
	universe_1.tick();
	universe_1.tick();
	// universe_1.tick();
	println!("{}", universe_1);
	// println!("{:?}", universe_1.cells[0]);

	// universe_1.get_new_index_with_direction(0, Direction::North.get_direction());
}
