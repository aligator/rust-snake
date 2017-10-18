use std::collections::LinkedList;
use logic::field::Point;

pub struct Snake {
    points: LinkedList<Point>,
    last_x: i32,
    last_y: i32,
	neck_point: Option<Point>,
	grow_point: Option<Point>
}

impl Snake {

    pub fn new(init_length: usize, pos_tail: Point, last_x: i32, last_y: i32) -> Snake {
        let mut points: LinkedList<Point> = LinkedList::new();

        for i in 0..(init_length) {
            points.push_front((pos_tail.0 + i, pos_tail.1));
        }

        Snake {
            points: points,
            last_x: last_x,
            last_y: last_y,
			neck_point: None,
			grow_point: None
        }
    }

	pub fn grow(&mut self) {
		
		self.grow_point = self.neck_point;
				
		
		//println!("{:?}", self.neck_point);
		//println!("{:?}", *self.get_head());
	}
	
    pub fn move_left(&mut self, width: i32, height: i32) -> bool { self.mov(-1, 0, width, height) }

    pub fn move_right(&mut self, width: i32, height: i32) -> bool {
        self.mov(1, 0, width, height)
    }

    pub fn move_up(&mut self, width: i32, height: i32) -> bool {
        self.mov(0, -1, width, height)
    }

    pub fn move_down(&mut self, width: i32, height: i32) -> bool {
        self.mov(0, 1, width, height)
    }

    pub fn move_last(&mut self, width: i32, height: i32) -> bool {
        self.mov(0, 0, width, height)
    }

    /**
    * if x and y are 0, the last direction is used
    *
    */
    fn mov(&mut self, x: i32,  y: i32, width: i32, height: i32) -> bool {
		{
			let mut iter = self.points.iter();
			iter.next(); // ignore head
			
			
			
			match iter.next() {// get seccond element
				Some(p) => self.neck_point = Some(*p),
				None => ()
			}
		
		}
		
        let (old_x, old_y) = *self.get_head();

        let x = if x == 0 && y == 0
            {self.last_x} else {x}; // if x=0 && y=0 => use last_x

        let y = if x == 0 && y == 0
            {self.last_y} else {y}; // if x=0 && y=0 => use last_y

        let x: i32 = (old_x) as i32 +x;
        let y: i32 = (old_y) as i32 +y;

        if x < 0 || y < 0 {
            return false;
        }




        // check for border
        if x >= width || y >= height {
            return false;
        }

        let new_pos = (x as usize, y as usize);

        // check for self-crash
        if self.get_points().contains(&new_pos) {
            return false;
        }


        self.points.push_front(new_pos);
		
		// grow 
		match self.grow_point {
			Some(p) => {
				if p == *self.get_tail() {
					self.grow_point = None;
				} else {
					self.points.pop_back();
				}
			},
			None => {self.points.pop_back();},
		}

        self.last_x = x;
        self.last_y = y;

        return true;
    }

    pub fn get_points(&self) -> &LinkedList<Point> {
        &self.points
    }

    pub fn get_head(&self) -> &Point {
        match self.points.front() {
            Some(i) => i,
            None => {
                panic!("Length of the snake is 0");
            }
        }
    }
	
	pub fn get_tail(&self) -> &Point {
        match self.points.back() {
            Some(i) => i,
            None => {
                panic!("Length of the snake is 0");
            }
        }
    }

    #[test]
    fn new_test() {
        let mut snake = Snake::new(3, (2,2), 1, 0);

        assert_eq!((4,2), *snake.get_head());
        assert_eq!((3,2), *snake.get_head());
        assert_eq!((2,2), *snake.get_head());
    }

    #[test]
    fn move_up_test() {

        let mut snake = Snake::new(3, (2,2), 1, 0);

        snake.move_up(5, 5);
        snake.move_left(5, 5);
        snake.move_down(5, 5);
        snake.move_right(5, 5);

        assert_eq!((4,1), *snake.get_head());
        assert_eq!((5,2), *snake.get_head());
        assert_eq!((5,1), *snake.get_head());
    }
}

