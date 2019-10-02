#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Creature {
	Empty = 0,
	Grass = 1,
	Sheep = 2,
	Wolf = 3,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Life {
	Dead = 0,
	Alive = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
	id:u32,
	creature: Creature,
	stamina:u8,
	status: Life
}

pub struct Organizm {
	creature: Creature,
	stamina:u8,
	status: Life
}

pub struct Universe {
	height: u32,
	width: u32,
	cells: Vec<Cell>
}
use std::cmp::Ordering;
impl Universe {
	fn get_index(&self, row: u32, column: u32) -> usize {
		(row * self.width + column) as usize
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

	pub fn new() -> Universe{
		let width = 64 as u32;
		let height = 64 as u32;
		let cells = (0..width*height).map(|i|{
			if i%2 == 0 || i%7 == 0{
				let new_object = Cell{
					id: i,
					creature: Creature::Grass,
					stamina: 0,
					status: Life::Dead
				};
				(new_object)
			}else{
				let new_object = Cell{
					id: i,
					creature: Creature::Empty,
					stamina: 0,
					status: Life::Dead
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

	pub fn create_creature(&mut self,column:u32,row:u32,new_creature: Organizm){
		let mut next = self.cells.clone();
		let cell_index = self.get_index(row,column);
		next[cell_index] = Cell{
			id: cell_index as u32,
			creature: new_creature.creature,
			stamina: new_creature.stamina,
			status: new_creature.status,
		};;
		// println!("{:?}",next[cell_index])
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
				if cell.creature == Creature::Empty { '_' }
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
	let mut universe_1 = Universe::new();
	let sheep_1 = Organizm{
		creature: Creature::Sheep,
		stamina: 20,
		status: Life::Alive
	};

	universe_1.create_creature(0,0,sheep_1);
	println!("{}",universe_1);
	universe_1.get_food_count(0,0);

}
