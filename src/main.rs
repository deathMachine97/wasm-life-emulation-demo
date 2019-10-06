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
	inside: Option<Inside>
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
	cells: Vec<Cell>
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
		let cells = (0..width*height).map(|i|{
			let index = i.clone() as usize;
			let position = get_coordinate(width, index);
			if i%2 == 0 || i%7 == 0{
				let new_object = Cell{
					id: i,
					position,
					creature: Creature::Grass,
					stamina: 0,
					status: Life::Dead,
					inside: None
				};
				(new_object)
			}else{
				let new_object = Cell{
					id: i,
					position,
					creature: Creature::Empty,
					stamina: 0,
					status: Life::Dead,
					inside: None
				};
				(new_object)
			}
		}).collect();
		Universe {
			width,
			height,
			cells,
		}
	}

	pub fn set(&mut self,new_creature: Organizm){
		let mut next = self.cells.clone();
		let cell_index = self.get_index(new_creature.position.row,new_creature.position.column);
		let position = get_coordinate(self.width, cell_index);
		next[cell_index] = Cell{
			id: next.len() as u32,
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
		};
		// println!("{:?}",next[cell_index]);
		self.cells = next;
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
				else if cell.creature == Creature::Grass { '*' }
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

// Движение
impl Universe{
	fn start_movements(&self){
		for cell in self.cells.iter(){
			if cell.status == Life::Alive{
				let cell_index = self.get_index_with_id(cell.id);
				match cell_index {
					Some(index) => {
						let eated_cell = match cell.inside {
							Some(e_cell) => Inside{
								id: cell.inside.id,
								creature: e_cell.inside.creature,
								position: e_cell.inside.position,
								stamina: e_cell.inside.stamina,
								status: e_cell.inside.status
							},
							None => Inside{
								id: index,
								creature: Creature::Empty,
								position: cell.position,
								stamina: 0,
								status: Life::Dead
							}
						};
						let will_eated_cell = 0;
						()
					},
					None => (),
				}
			}
			//
			// let object_id = object_id as u32;
			// let cell_index: Option<usize> = self.get_index_with_id(object_id);
			// match cell_index {
			// 	Some(index)=> {
			// 		let cell:Cell = self.cells[index];
			// 		println!("{:?}",cell );
			// 		if cell.creature == Creature::Sheep || cell.creature == Creature::Wolf{
			// 			println!("{:?}", cell);
			// 		}
			// 	},
			// 	None => ()
			// };
		}
	}
}


// impl Universe {
// 	pub fn tick(&self){
// 		let mut next = &self.cells;
// 		for row in 0..self.height{
// 			for col in 0..self.width{
//
// 			}
// 		}
// 	}
// }

fn main() {
	let mut universe_1 = Universe::new(5,5);
	let sheep_1 = Organizm{
		position: Position{column:1,row:0},
		creature: Creature::Sheep,
		stamina: 20,
		status: Life::Alive
	};
	universe_1.set(sheep_1);
	println!("{}",universe_1);
	universe_1.start_movements();
}
