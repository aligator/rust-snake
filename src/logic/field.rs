extern crate rand;

use logic::snake::Snake;
use std::collections::LinkedList;
use logic::field::rand::distributions::IndependentSample;

#[derive(Copy, Debug)]
pub enum Direction {
    Left, Right, Up, Down
}

impl Clone for Direction {
    fn clone(&self) -> Direction { *self }
}

pub type Point = (usize, usize); // Positions of the snake-parts (x, y)

pub struct Field<T:Clone> {
    field: Vec<Vec<T>>,
    default_val: T,
    snake_val: T,
    head_val: T,
    cookie_val: T,
    width: usize,
    height: usize,
    snake: Snake,
    cookie: Point
}

#[allow(dead_code)]
impl<T:Clone> Field<T> {

    pub fn new(width: usize, height: usize, default_val: T, snake_val: T, head_val: T, cookie_val: T, snake_size: usize) -> Option<Field<T>> {

        let start_x = width / 2 - snake_size / 2;
        let start_y = height / 2;

        let snake = Snake::new(snake_size, (start_x, start_y), 1, 0);

        if snake.get_head().0 >= width {
            return None;
        }
		
		let mut field = Field {
            field: vec![vec![default_val.clone(); width as usize]; height as usize],
            default_val: default_val.clone(),
            snake_val: snake_val.clone(),
            head_val: head_val.clone(),
            cookie_val: cookie_val.clone(),
            width: width,
            height: height,
            snake: snake,
            cookie: (0, 0),
        };
		
		field.new_cookie();
		
        Some(field)
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn default_val(&self) -> &T {
        &self.default_val
    }

    pub fn get_snake_val(&self) -> &T {
        &self.snake_val
    }

    pub fn get_head_val(&self) -> &T {
        &self.head_val
    }

    pub fn get_field(&self) -> Vec<Vec<T>> {
        let mut field = self.field.clone();
        for (point, chr) in self.get_snake_with_chars() {
			let p = util::limit_point(point, self.width, self.height);
            field[p.1][p.0] = chr;
        }
		
		let p = util::limit_point(self.cookie, self.width, self.height);
        field[p.1][p.0] = self.cookie_val.clone();
		
        field
    }

    pub fn get_snake(&self) -> &Snake {
        &self.snake
    }

    pub fn reset_field(&mut self) {
        self.field = vec![vec![self.default_val.clone(); self.width]; self.height];
    }

    pub fn set_point(&mut self, x:usize, y:usize, data: T) {
        let p = util::limit_point((x, y), self.width, self.height);
        
        self.field[p.1][p.0] = data;
    }

    pub fn get_point(&self, x:usize, y:usize) -> &T {
        let p = util::limit_point((x, y), self.width, self.height);

        &self.field[p.1][p.0]
    }

    fn new_cookie(&mut self) {
        let x_rng = rand::distributions::Range::new(0, self.width);
        let y_rng = rand::distributions::Range::new(0, self.height);
        let mut rng = rand::thread_rng();
				
		loop {
			let x = x_rng.ind_sample(&mut rng);
			let y = y_rng.ind_sample(&mut rng);
			
			if !self.snake.contains(x, y) { 
				self.cookie = (x, y);
				break;
			}
		}

		
        
    }

    pub fn mov(&mut self, dir: Option<Direction>) -> (Option<LinkedList<(Point, T)>>, bool) {

        let old_tail = *self.snake.get_points().back().expect("There is no snake.");
        let old_front = *self.snake.get_points().front().expect("There is no snake.");

        let (move_ok, score) = match dir {

            Some(Direction::Left) => self.snake.move_left(self.width as i32, self.height as i32),
            Some(Direction::Right) => self.snake.move_right(self.width as i32, self.height as i32),
            Some(Direction::Up) => self.snake.move_up(self.width as i32, self.height as i32),
            Some(Direction::Down) => self.snake.move_down(self.width as i32, self.height as i32),
            None => self.snake.move_last(self.width as i32, self.height as i32),
        };

        if move_ok {
		
			
		
            let mut to_update: LinkedList<(Point, T)> = LinkedList::new();
            to_update.push_front((old_tail, self.default_val.clone()));
            to_update.push_front((old_front, self.snake_val.clone()));			
            to_update.push_front((*self.snake.get_points().front().expect("There is no snake."), self.head_val.clone()));
						
			if *self.snake.get_head() == self.cookie {
				self.new_cookie();
				self.snake.grow();
				to_update.push_front((self.cookie, self.cookie_val.clone()));
			}
			
            (Some(to_update), score)
        } else {
            (None, score)
        }

    }

    fn get_snake_with_chars(&self) -> LinkedList<(Point, T)> {

        let mut snake: LinkedList<(Point, T)> = LinkedList::new();
        for &item in self.snake.get_points() {
            snake.push_front((item, self.snake_val.clone()));
        }

        let (head, _) = snake.pop_back().expect("There is no snake.");

        snake.push_back((head, self.head_val.clone()));

        snake
    }
}

mod util {
    pub fn limit<L:PartialOrd>(val: L, min: L, max: L) -> L {
        if val < min {
            min
        } else if val > max {
            max
        } else {
            val
        }
    }
	
	pub fn limit_point(point: super::Point, width: usize, height: usize) -> super::Point {
		let x = limit(point.0, 0, width);
        let y = limit(point.1, 0, height);
		
		(x, y)
	}
}


#[cfg(test)]
mod field_test {
    use logic::Field;

    #[test]
    fn new_field() {
        let field = Field::new(3, 5, false);
        let field = field.get_field();


        assert_eq!(5, field.len()); // testing width
        assert_eq!(3, field[0].len()); // testing height

        for col in field.iter() {
            for item in col.iter() {
                let item = *item;
                assert_eq!(false, item); // test if everything is false
            }
        }
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 5 but the index is 5")]
    fn set_get_point_test() {
        let mut field = Field::new(3, 5, false);

        field.set_point(0,0,true);
        assert_eq!(true, *field.get_point(0,0));

        field.set_point(2,4,true);
        assert_eq!(true, *field.get_point(2,4));


        field.set_point(3,5,true);
    }

    #[test]
    fn reset_field_test() {
        let mut field = Field::new(3, 5, false);

        field.set_point(0,0,true);
        field.set_point(1,3,true);
        field.set_point(2,4,true);

        field.reset_field();

        for col in field.get_field().iter() {
            for item in col.iter() {
                let item = *item;
                assert_eq!(false, item); // test if everything is false
            }
        }
    }
}
