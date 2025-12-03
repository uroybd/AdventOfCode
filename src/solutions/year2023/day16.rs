// Advent of Code 2023 - Day 16
use rayon::prelude::*;
use std::sync::mpsc::channel;
use std::{collections::HashSet, fs};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum MovementDirection {
    Rightward,
    LeftWard,
    Upward,
    Downward,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum RoomTile {
    Empty,
    RightTiltedMirror,
    LeftTiltedMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl std::convert::From<char> for RoomTile {
    fn from(s: char) -> Self {
        match s {
            '.' => Self::Empty,
            '/' => Self::RightTiltedMirror,
            '\\' => Self::LeftTiltedMirror,
            '|' => Self::VerticalSplitter,
            '-' => Self::HorizontalSplitter,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Photon {
    position: (isize, isize),
    direction: MovementDirection,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct MirrorRoom {
    room: Vec<Vec<RoomTile>>,
    width: isize,
    height: isize,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMirrorRoomError;

impl std::str::FromStr for MirrorRoom {
    type Err = ParseMirrorRoomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let room: Vec<Vec<RoomTile>> = s
            .lines()
            .map(|l| l.chars().map(RoomTile::from).collect())
            .collect();
        let width = room[0].len() as isize;
        let height = room.len() as isize;
        Ok(Self {
            room,
            width,
            height,
        })
    }
}

impl MirrorRoom {
    fn get_next_photon(&self, photon: &Photon) -> Option<Vec<Photon>> {
        let new_pos = match photon.direction {
            MovementDirection::Rightward => (photon.position.0 + 1, photon.position.1),
            MovementDirection::LeftWard => (photon.position.0 - 1, photon.position.1),
            MovementDirection::Upward => (photon.position.0, photon.position.1 - 1),
            MovementDirection::Downward => (photon.position.0, photon.position.1 + 1),
        };
        if new_pos.0 < 0 || new_pos.0 >= self.width || new_pos.1 < 0 || new_pos.1 >= self.height {
            return None;
        }
        let tile = &self.room[new_pos.1 as usize][new_pos.0 as usize];
        let mut directions = vec![];

        match tile {
            RoomTile::Empty => directions.push(photon.direction.clone()),
            RoomTile::RightTiltedMirror => directions.push(match photon.direction {
                MovementDirection::Rightward => MovementDirection::Upward,
                MovementDirection::LeftWard => MovementDirection::Downward,
                MovementDirection::Upward => MovementDirection::Rightward,
                MovementDirection::Downward => MovementDirection::LeftWard,
            }),
            RoomTile::LeftTiltedMirror => directions.push(match photon.direction {
                MovementDirection::Rightward => MovementDirection::Downward,
                MovementDirection::LeftWard => MovementDirection::Upward,
                MovementDirection::Upward => MovementDirection::LeftWard,
                MovementDirection::Downward => MovementDirection::Rightward,
            }),
            RoomTile::VerticalSplitter => match &photon.direction {
                MovementDirection::Rightward | MovementDirection::LeftWard => {
                    directions.extend([MovementDirection::Upward, MovementDirection::Downward])
                }
                dir => directions.push(dir.clone()),
            },
            RoomTile::HorizontalSplitter => match &photon.direction {
                MovementDirection::Upward | MovementDirection::Downward => {
                    directions.extend([MovementDirection::Rightward, MovementDirection::LeftWard])
                }
                dir => directions.push(dir.clone()),
            },
        }
        Some(
            directions
                .into_iter()
                .map(|dir| Photon {
                    position: new_pos,
                    direction: dir,
                })
                .collect(),
        )
    }

    fn find_photons(&self, starter: &Photon) -> usize {
        let mut available = vec![];
        let mut seen = HashSet::new();
        if let Some(next_photons) = self.get_next_photon(starter) {
            available.extend(next_photons);
        }
        while let Some(photon) = available.pop() {
            if seen.contains(&photon) {
                continue;
            }
            seen.insert(photon.clone());
            if let Some(next_photons) = self.get_next_photon(&photon) {
                available.extend(next_photons);
            }
        }
        let pos_counter: HashSet<(isize, isize)> =
            HashSet::from_iter(seen.iter().map(|p| p.position));

        pos_counter.len()
    }
}

pub fn solution_day_16_01(file_path: String) -> Option<usize> {
    let mirror_room: MirrorRoom = fs::read_to_string(file_path).unwrap().parse().unwrap();
    Some(mirror_room.find_photons(&Photon {
        position: (-1, 0),
        direction: MovementDirection::Rightward,
    }))
}

pub fn solution_day_16_02(file_path: String) -> Option<usize> {
    let mirror_room: MirrorRoom = fs::read_to_string(file_path).unwrap().parse().unwrap();

    let (sender, receiver) = channel();
    (0..mirror_room.width)
        .into_par_iter()
        .for_each_with(sender, |s, x| {
            s.send(mirror_room.find_photons(&Photon {
                position: (x, -1),
                direction: MovementDirection::Downward,
            }))
            .unwrap();
            s.send(mirror_room.find_photons(&Photon {
                position: (x, mirror_room.height),
                direction: MovementDirection::Upward,
            }))
            .unwrap();
        });
    let (sender2, receiver2) = channel();
    (0..mirror_room.height)
        .into_par_iter()
        .for_each_with(sender2, |s, y| {
            s.send(mirror_room.find_photons(&Photon {
                position: (-1, y),
                direction: MovementDirection::Rightward,
            }))
            .unwrap();
            s.send(mirror_room.find_photons(&Photon {
                position: (mirror_room.width, y),
                direction: MovementDirection::LeftWard,
            }))
            .unwrap();
        });

    Some(
        receiver
            .iter()
            .max()
            .unwrap()
            .max(receiver2.iter().max().unwrap()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_16_01() {
        let file_path: String = String::from("inputs/2023/day16e.txt");
        let result = solution_day_16_01(file_path).unwrap();
        assert_eq!(result, 46);
    }

    #[test]
    fn test_day_16_02() {
        let file_path: String = String::from("inputs/2023/day16e.txt");
        let result = solution_day_16_02(file_path).unwrap();
        assert_eq!(result, 51);
    }

    #[test]
    #[ignore]
    fn output_day_16_01() {
        let file_path: String = String::from("inputs/2023/day16.txt");
        let result = solution_day_16_01(file_path).unwrap();
        assert_eq!(result, 8098);
    }

    #[test]
    #[ignore]
    fn output_day_16_02() {
        let file_path: String = String::from("inputs/2023/day16.txt");
        let result = solution_day_16_02(file_path).unwrap();
        assert_eq!(result, 8335);
    }
}
