pub mod maze_engine {
    use std::{thread::sleep, time::Duration, process::Command};


    #[derive(Debug, Clone)]
    enum PositionPoist {
        Previos((i32, i32)),
        Now((i32, i32))
    }

    pub struct MazeEngine{
        stack: Vec<(i32, i32)>,
        processed: Vec<(i32, i32)>,
        processing: Option<(i32, i32)>,
        start_position: (i32, i32),
        end_position: (i32, i32),
        chains_routes: Vec<((i32, i32), (i32, i32))>,
        // opt
        live_streaming: (bool, u32),
    }

    impl MazeEngine {
        pub fn new(start_position:(i32, i32), end_position:(i32, i32), live_streaming: (bool, u32))-> Self{
            MazeEngine{
                stack: Vec::new(),
                processed: Vec::new(),
                processing: None,
                start_position,
                end_position,
                chains_routes: Vec::new(),
                // opt
                live_streaming,
            }
        }

        pub fn running_in_maze(&mut self, maze: &Vec<Vec<char>>){

            let first_position = self.start_position;
            self.stack.push(first_position);

            // for _ in 0..10 {
            //     // if self.stack.len() <= 0 {
            //     //     break;
            //     // }

            //     println!("{:?}", self.stack);
            //     self.check_area(maze);                
            // }

            loop {
                if self.stack.len() <= 0 {
                    break;
                }

                self.check_area(maze);

                if self.live_streaming.0{
                    sleep(Duration::from_millis(self.live_streaming.1 as u64));
                    self.live_stream_fn(maze.clone());
                }

                // stop if have reached the end position
                if self.processing.unwrap() == self.end_position{
                    println!("found");
                    break;
                }          
            }

            let route = self.calculate_chains();
            self.show_route(route, &maze);

        }

        pub fn live_stream_fn(&self,mut maze: Vec<Vec<char>>){
            Command::new("clear")
            .status()
            .unwrap();
            
            for cordinate in &self.processed {
                maze[cordinate.0 as usize][cordinate.1 as usize] = '@';
            }
            self.show_maze(&maze);
        }

        pub fn show_route(&self, route: Vec<(i32, i32)>, maze: &Vec<Vec<char>>){
            let mut maze = maze.clone();
            for (coll, row) in route {
                maze[coll as usize][row as usize] = '@';

                if self.live_streaming.0{
                    Command::new("clear")
                    .status()
                    .unwrap();

                self.show_maze(&maze);        
                sleep(Duration::from_millis(25));
                }

            }

            if !self.live_streaming.0{
                self.show_maze(&maze);
            }

        }

        pub fn show_maze(&self, maze: &Vec<Vec<char>>){
            for colls in maze {
                println!("{:?}", colls);
            }
        }

        fn calculate_chains(&self)-> Vec<(i32, i32)>{
            let mut final_routes= Vec::new();
            let mut position_save = self.end_position;

            loop {
                if position_save == self.start_position{
                    break;
                }
                
                let point = self.chains_routes.clone()
                .into_iter()
                .find(|(_value_prvious, value_now)|{
                    
                    if position_save == *value_now{
                        true
                    } else {
                        false
                    }

                })
                .expect("cordinate not found");

            final_routes.push(point.0);
            position_save = point.0;
        }

        final_routes.reverse();
        final_routes.push(self.end_position);
        return final_routes;

            // final_routes.push(point);
        }

        fn check_area(&mut self, maze: &Vec<Vec<char>>){
            let processing = self.stack.pop()
                .unwrap();
            let values_of_check = [1, -1];

            self.processed.push(processing);
            self.processing = Some(processing);

            // colls
            for check in &values_of_check {
                let colls_pos = processing.0 + check;
                let row_pos = processing.1;
                if colls_pos >= 0 && colls_pos < maze.len() as i32{
                    let value_contains = maze[colls_pos as usize][row_pos as usize];
                    let cordinate = (colls_pos, row_pos);
                    if value_contains != '#' && !self.processed.contains(&cordinate){
                        
                        let position_point = (
                            processing,
                            cordinate
                        );
                        self.chains_routes.push(position_point);

                        self.stack.push(cordinate);
                    }
                }
            }

            // rows
            for check in &values_of_check {
                let colls_pos = processing.0 ;
                let row_pos = processing.1 + check;
                if row_pos >= 0 && row_pos < maze[colls_pos as usize].len() as i32{
                    let value_contains = maze[colls_pos as usize][row_pos as usize];
                    let cordinate = (colls_pos, row_pos);
                    if value_contains != '#' && !self.processed.contains(&cordinate){

                        let position_point = (
                            processing,
                            cordinate,
                        );
                        self.chains_routes.push(position_point);

                        self.stack.push(cordinate);
                    }
                }
            }
        }
    }
}