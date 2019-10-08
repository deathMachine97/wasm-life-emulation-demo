// Добавить параметр calories для cell
// Добавить параметр health для cell
//
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Inside {
	id:u32,
	creature: Creature,
	position: Position,
	stamina:u8,
	status: Life,
	calories: f32,
	health: u8
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
	id:u32,
	creature: Creature,
	position: Position,
	stamina:u8,
	status: Life,
	inside: Option<Inside>,
	calories: f32,
	direction: Direction,
	health: u8
}

#[derive(Debug)]
pub struct Organizm {
	position: Position,
	creature: Creature,
	stamina:u8,
	status: Life,
	calories: f32,
	health: u8
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

impl Creature {
	fn get_digestion_rate(&self) -> f32{
		match &self {
			Creature::Wolf => {2.0},
			Creature::Sheep => {0.5},
			_otherwisee => 0.0
		}
	}

	fn get_eating_food_type(&self) -> Creature{
		match self {
			Creature::Sheep => Creature::Grass,
			Creature::Wolf => Creature::Sheep,
			_otherwisee => Creature::Empty
		}
	}

	fn rot(&self) -> f32{
		match self {
			Creature::Sheep => 15.0,
			Creature::Wolf => 10.0,
			Creature::Grass => 0.0,
			Creature::Empty => 0.0
		}
	}
}

impl Direction {
	fn get_direction_coordinate(&self) -> (i32,i32){
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
					direction: Direction::Stand,
					calories: 10.0,
					health: 0
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
					direction: Direction::Stand,
					calories: 0.0,
					health: 0
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
				status: next[cell_index].status,
				calories: next[cell_index].calories,
				health: 0
			}),
			direction: Direction::Stand,
			calories: new_creature.calories,
			health: new_creature.health
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
				if cell.creature == Creature::Empty { '.' }
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
					stamina: 0,
					status: Life::Dead,
					inside: None,
					direction: Direction::Stand,
					calories: inside_cell.calories + self.creature.rot(),
					health: 0
				}
			}
			None => {
				Cell{
					id: self.id,
					creature: Creature::Grass,
					position: self.position,
					stamina: 0,
					status: Life::Dead,
					inside: None,
					direction: Direction::Stand,
					calories: self.creature.rot(),
					health: 0
				}
			}
		};
	}

	fn heal(&mut self,heal_size:u8){
		if self.calories > ((heal_size as f32) / 1.5) + 1.0 {
			self.health = self.health + heal_size*2;
			self.calories = self.calories - (heal_size as f32) / 1.5;
		}
	}

	fn take_damage(&mut self, damage_size:u8){
		if self.health > damage_size{
			self.health = self.health - damage_size;
		}
		else{
			self.health = 0;
		}

	}

	fn increase_stamina(&mut self, stamina_size: u8){
		self.stamina = self.stamina + stamina_size;
		self.calories = self.calories - stamina_size as f32;
	}

	fn balance_energy(&mut self){
		self.feed();
		if self.stamina <= 1 && self.calories > 1.0{
			self.increase_stamina(1);
		}

		if self.health < 100 && self.calories > 2.0 {
			if 100 - self.health >= 10{
				self.heal(5);
			}else{
				self.heal(1);
			}
		}

		if self.calories <= 0.0 {
			self.calories = 0.0;
			self.take_damage(10)
		}

		if self.health == 0{
			self.kill_cell();
		}

	}

	fn feed(&mut self){
		match  self.inside {
			Some(mut inside) =>{
				let mut next = self.clone();
				if inside.creature == self.creature.get_eating_food_type(){
					if inside.calories > 0.0 {
						let digestion_rate = if inside.calories < next.creature.get_digestion_rate(){inside.calories} else {next.creature.get_digestion_rate()};
						if inside.calories - digestion_rate > 0.0{
							next.calories = next.calories + digestion_rate;
							inside.calories = inside.calories - digestion_rate;
						}else{
							next.calories = next.calories + inside.calories;
							inside.calories = 0.0;
						}

					}

					if inside.calories <= 0.0{
						next.inside = None;
					}else{
						next.inside = Some(inside);
					}

					*self = next;
				}
			},
			None=>()
		}
	}

	fn pack_to_inside(&self)->Option<Inside>{
		Some(Inside{
			id: self.id,
			creature: self.creature,
			position: self.position,
			stamina:self.stamina,
			status: self.status,
			calories: self.calories,
			health: 0
		})
	}

	fn get_inside_cell(&self)->Option<Inside>{
		let inside: Option<Inside> = match self.inside {
			Some(inside)=>{
				Some(Inside{
					id:inside.id,
					creature: inside.creature,
					position: inside.position,
					stamina:inside.stamina,
					status: inside.status,
					calories: inside.calories,
					health: inside.health
				})
			},
			None=> None
		};
		inside
	}

	fn spin_the_inside(&self,spinned_cell_index:usize,spinned_cell_position:Position)->Cell{
		match self.inside{
			Some(e_cell) => Cell{
				id: e_cell.id,
				creature: e_cell.creature,
				position: e_cell.position,
				stamina: e_cell.stamina,
				status: e_cell.status,
				inside: None,
				direction: Direction::Stand,
				calories: e_cell.calories,
				health: 0
			},
			None => Cell{
				id: spinned_cell_index as u32,
				creature: Creature::Empty,
				position: spinned_cell_position,
				stamina: 0,
				status: Life::Dead,
				inside: None,
				direction: Direction::Stand,
				calories: 0.0,
				health: 0
			}
		}
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
			if cell.status == Life::Alive && cell.stamina >= 1{
				let cell_index = self.get_index_with_id(cell.id);
				match cell_index {
					Some(old_index) => {
						let chosen_direction:Direction = Direction::NorthWest;
						let direction: (i32,i32) = chosen_direction.get_direction_coordinate();
						let new_index: usize = self.get_new_index_with_direction(old_index, direction);
						let inside_in_new_place: Option<Inside> =
							if chosen_direction == Direction::Stand {	self.cells[old_index].get_inside_cell()	}
							else {	self.cells[new_index].pack_to_inside()	};
						let minus_stamina = if chosen_direction == Direction::Stand {0} else {1};
						let creature_in_new_place = self.create_cell_for_new_position(new_index,old_index, inside_in_new_place,minus_stamina);
						let inside_in_old_place: Cell = cell.spin_the_inside(old_index,cell.position);
						next[old_index] = inside_in_old_place;
						next[new_index] = creature_in_new_place;
						()
					},
					None => (),
				}
			}
		}
		self.cells = next;
	}

	fn create_cell_for_new_position(&self, new_index: usize,old_index: usize, inside_in_new_place: Option<Inside>,minus_stamina:u8) -> Cell{
		Cell{
			id: self.cells[old_index].id,
			creature: self.cells[old_index].creature,
			position: self.cells[new_index].position,
			stamina: self.cells[old_index].stamina - minus_stamina,
			status: self.cells[old_index].status,
			inside: inside_in_new_place,
			direction: Direction::Stand,
			calories: self.cells[old_index].calories,
			health: self.cells[old_index].health
		}
	}

	pub fn tick(&mut self){
		let mut next = self.cells.clone();
		for cell in self.cells.iter() {
			if cell.status == Life::Alive{
				match self.get_index_with_id(cell.id){
					Some(cell_index) => {
						next[cell_index].balance_energy();
					}
					None => ()
				}
			}
		}
		self.cells = next;
		self.make_movement();
	}

	fn analyze_creatures(&self,creature_type:Creature){
		for cell in self.cells.iter() {
			if cell.creature == creature_type{
				match self.get_index_with_id(cell.id){
					Some(index) => {
						println!("creature: {:?}\ncreature_id: {:?}\nstamina: {:?}\nstatus: {:?}\ncalories: {:?}\nhealth: {:?}\n\n", self.cells[index].creature,self.cells[index].id,self.cells[index].stamina,self.cells[index].status, self.cells[index].calories, self.cells[index].health);
						println!("{:?}",self.cells[index]);
					}
					None => ()
				}
			}
		}
	}
}


fn main() {
	let mut universe_1 = Universe::new(5,6);
	let sheep_1 = Organizm{
		position: Position{column:0,row:0},
		creature: Creature::Sheep,
		stamina: 10,
		status: Life::Alive,
		calories: 20.0,
		health: 100
	};
	universe_1.set(sheep_1);
	for _ in 0..61 {
		universe_1.tick();
		println!("{}",universe_1 );
	}

	universe_1.analyze_creatures(Creature::Sheep);
	// universe_1.get_new_index_with_direction(0, Direction::North.get_direction_coordinate());
}
