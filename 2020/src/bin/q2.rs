use std::io;
use std::collections::HashMap;

#[derive(Clone)]
struct Room {
    visits: u32,
    connections: HashMap<usize, u32>,
    rooms: Vec<usize>,
}

impl Room {
    fn new() -> Self {
        Self { visits: 0, connections: HashMap::new(), rooms: Vec::new() }
    }

    fn connect(&mut self, other: usize) {
        self.connections.insert(other, 0);

        if let Err(pos) = self.rooms.binary_search(&other) {
            self.rooms.insert(pos, other);
        } else {
            panic!("Room {} already connected", other);
        }
    }

    // returns next room
    fn visit(&mut self) -> usize {
        self.visits += 1;

        let next_room = 
            if self.visits % 2 == 0 {
                let odd_exits = self.rooms
                    .iter()
                    .enumerate()
                    .filter(|(_, room)| self.connections[room] % 2 == 1);

                let (i, _) = odd_exits
                    .min_by_key(|(_, id)| **id)
                    .unwrap();

                if i == self.rooms.len() - 1 { self.rooms[i] } else { self.rooms[i+1] }

            } else {
                self.rooms[0]
            };

        *self.connections.get_mut(&next_room).unwrap() += 1;

        next_room
    }

    fn print_connections(&self) {
        let mut letters = self.connections.keys()
            .map(|id| id_to_char(*id))
            .collect::<Vec<_>>();

        letters.sort();

        println!("{}", letters.into_iter().collect::<String>());
    }
}

struct Complex {
    rooms: Vec<Room>,
    current_room: usize,
}

impl Complex {
    fn plan_from_str(plan_str: &str) -> Vec<usize> {
        plan_str.trim().chars().map(|c| c as usize - 'A' as usize).rev().collect()
    }

    fn new(plan_str: &str) -> Self {
        let mut plan = Self::plan_from_str(plan_str);
        let mut unused: Vec<usize> = (0..plan.len()+2).collect();

        let mut rooms = vec![Room::new(); plan.len()+2];

        while !plan.is_empty() {
            for &id1 in &unused {
                if !plan.contains(&id1) {
                    let id2 = plan.pop().unwrap();

                    rooms[id1].connect(id2);
                    rooms[id2].connect(id1);

                    unused.retain(|room_id| *room_id != id1);
                    break;
                }
            }
        }

        assert_eq!(unused.len(), 2);

        rooms[unused[0]].connect(unused[1]);
        rooms[unused[1]].connect(unused[0]);

        Self { rooms, current_room: 0 }
    }

    fn move_spy(&mut self) {
        self.current_room = self.rooms[self.current_room].visit();
    }

    fn print_rooms(&self) {
        for room in &self.rooms {
            room.print_connections();
        }
    }
}

fn id_to_char(id: usize) -> char {
    char::from_u32(id as u32 + 0x41).unwrap()
}


fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let plan_str = input.split_whitespace().next().unwrap();
    let mut tokens = input
        .split_whitespace()
        .skip(1)
        .map(|substr| substr.parse::<u32>().unwrap());

    let p = tokens.next().unwrap();
    let q = tokens.next().unwrap();

    let mut complex = Complex::new(plan_str);
    complex.print_rooms();

    for _ in 0..p {
        complex.move_spy();
    }
    print!("{}", id_to_char(complex.current_room));

    for _ in 0..q-p {
        complex.move_spy();
    }
    println!("{}", id_to_char(complex.current_room));

    Ok(())
}

