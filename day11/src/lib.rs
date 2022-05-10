mod vec_2d;
use vec_2d::Vec2D;

pub fn dumbo_octopi(data: Vec<usize>, width: usize, steps: usize) -> usize {
    let mut energy_levels = Vec2D::from_vec(data, width);
    let mut total_flashes = 0;
    for _step in 0..steps {
        let mut can_flash = vec![true; width * width];
        increment(&mut energy_levels);

        while is_any_energy_level_greater_than_nine(&mut energy_levels) {
            let result = flash(energy_levels, &mut can_flash);

            energy_levels = result.0;
            total_flashes += result.1;
        }
    }

    total_flashes
}

pub fn loop_until_all_flash(data: Vec<usize>, width: usize) -> usize {
    let mut energy_levels = Vec2D::from_vec(data, width);
    let mut steps = 0;

    loop {
        let mut can_flash = vec![true; width * width];
        increment(&mut energy_levels);

        while is_any_energy_level_greater_than_nine(&mut energy_levels) {
            let result = flash(energy_levels, &mut can_flash);

            energy_levels = result.0;
        }

        steps += 1;

        if can_flash.iter().fold(false, |acc, e| acc | e) == false {
            break;
        }
    }

    steps
}

fn is_any_energy_level_greater_than_nine(energy_levels: &mut Vec2D) -> bool {
    energy_levels.iter().any(|e| *e > 9)
}

fn increment(vector: &mut Vec2D) {
    for x in 0..vector.width {
        for y in 0..vector.width {
            vector.add(x, y, 1);
        }
    }
}

fn flash(energy_levels: Vec2D, can_flash: &mut Vec<bool>) -> (Vec2D, usize) {
    let width = energy_levels.width;
    let mut new_energy_levels = energy_levels.clone();
    let mut flashes = 0;

    for y in 0..width {
        for x in 0..width {
            let idx = x + y * width;
            if energy_levels.get(x, y) > 9 && can_flash[idx] {
                flashes += 1;
                new_energy_levels.set(x, y, 0);
                can_flash[idx] = false;
                increment_neighbours(x, y, &mut new_energy_levels, can_flash);
            }
        }
    }

    (new_energy_levels, flashes)
}

fn increment_neighbours(
    x: usize,
    y: usize,
    new_energy_levels: &mut Vec2D,
    can_flash: &mut Vec<bool>,
) {
    let width = new_energy_levels.width;
    if x + 1 < width {
        if can_flash[(x + 1) + y * width] {
            new_energy_levels.add(x + 1, y, 1);
        }
    }
    if x >= 1 {
        if can_flash[(x - 1) + y * width] {
            new_energy_levels.add(x - 1, y, 1);
        }
    }
    if y + 1 < width {
        if can_flash[x + (y + 1) * width] {
            new_energy_levels.add(x, y + 1, 1);
        }
    }
    if y >= 1 {
        if can_flash[x + (y - 1) * width] {
            new_energy_levels.add(x, y - 1, 1);
        }
    }
    if x + 1 < width && y + 1 < width {
        if can_flash[(x + 1) + (y + 1) * width] {
            new_energy_levels.add(x + 1, y + 1, 1);
        }
    }
    if x >= 1 && y >= 1 {
        if can_flash[(x - 1) + (y - 1) * width] {
            new_energy_levels.add(x - 1, y - 1, 1);
        }
    }
    if x + 1 < width && y >= 1 {
        if can_flash[(x + 1) + (y - 1) * width] {
            new_energy_levels.add(x + 1, y - 1, 1);
        }
    }
    if x >= 1 && y + 1 < width {
        if can_flash[(x - 1) + (y + 1) * width] {
            new_energy_levels.add(x - 1, y + 1, 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{dumbo_octopi, loop_until_all_flash};

    #[test]
    fn test_part1() {
        let data = vec![
            5, 4, 8, 3, 1, 4, 3, 2, 2, 3, 2, 7, 4, 5, 8, 5, 4, 7, 1, 1, 5, 2, 6, 4, 5, 5, 6, 1, 7,
            3, 6, 1, 4, 1, 3, 3, 6, 1, 4, 6, 6, 3, 5, 7, 3, 8, 5, 4, 7, 8, 4, 1, 6, 7, 5, 2, 4, 6,
            4, 5, 2, 1, 7, 6, 8, 4, 1, 7, 2, 1, 6, 8, 8, 2, 8, 8, 1, 1, 3, 4, 4, 8, 4, 6, 8, 4, 8,
            5, 5, 4, 5, 2, 8, 3, 7, 5, 1, 5, 2, 6,
        ];
        const WIDTH: usize = 10;
        const STEPS: usize = 100;

        let total_flashes = dumbo_octopi(data, WIDTH, STEPS);
        assert_eq!(total_flashes, 1656);
    }

    #[test]
    fn test_part2() {
        let data = vec![
            5, 4, 8, 3, 1, 4, 3, 2, 2, 3, 2, 7, 4, 5, 8, 5, 4, 7, 1, 1, 5, 2, 6, 4, 5, 5, 6, 1, 7,
            3, 6, 1, 4, 1, 3, 3, 6, 1, 4, 6, 6, 3, 5, 7, 3, 8, 5, 4, 7, 8, 4, 1, 6, 7, 5, 2, 4, 6,
            4, 5, 2, 1, 7, 6, 8, 4, 1, 7, 2, 1, 6, 8, 8, 2, 8, 8, 1, 1, 3, 4, 4, 8, 4, 6, 8, 4, 8,
            5, 5, 4, 5, 2, 8, 3, 7, 5, 1, 5, 2, 6,
        ];
        const WIDTH: usize = 10;

        let steps = loop_until_all_flash(data, WIDTH);
        assert_eq!(steps, 195);
    }

    #[test]
    fn small_example() {
        let data = vec![
            1, 1, 1, 1, 1, 1, 9, 9, 9, 1, 1, 9, 1, 9, 1, 1, 9, 9, 9, 1, 1, 1, 1, 1, 1,
        ];
        const WIDTH: usize = 5;
        const STEPS: usize = 2;

        let total_flashes = dumbo_octopi(data, WIDTH, STEPS);
        assert_eq!(total_flashes, 9);
    }

    #[test]
    fn day11_part1() {
        let data = vec![
            2, 5, 6, 6, 8, 8, 5, 4, 3, 2, 3, 8, 5, 7, 4, 1, 4, 3, 5, 7, 6, 7, 6, 1, 5, 4, 3, 2, 4,
            7, 5, 4, 7, 7, 3, 3, 2, 1, 1, 4, 3, 7, 3, 1, 5, 8, 5, 3, 8, 5, 1, 7, 1, 6, 7, 8, 3, 1,
            7, 3, 1, 2, 7, 7, 3, 2, 1, 6, 1, 2, 3, 3, 7, 1, 1, 7, 6, 1, 4, 8, 1, 1, 6, 2, 5, 7, 8,
            2, 8, 5, 6, 1, 4, 4, 7, 2, 6, 3, 6, 7,
        ];

        const WIDTH: usize = 10;
        const STEPS: usize = 100;

        let total_flashes = dumbo_octopi(data, WIDTH, STEPS);
        assert_eq!(total_flashes, 1647);
    }

    #[test]
    fn day11_part2() {
        let data = vec![
            2, 5, 6, 6, 8, 8, 5, 4, 3, 2, 3, 8, 5, 7, 4, 1, 4, 3, 5, 7, 6, 7, 6, 1, 5, 4, 3, 2, 4,
            7, 5, 4, 7, 7, 3, 3, 2, 1, 1, 4, 3, 7, 3, 1, 5, 8, 5, 3, 8, 5, 1, 7, 1, 6, 7, 8, 3, 1,
            7, 3, 1, 2, 7, 7, 3, 2, 1, 6, 1, 2, 3, 3, 7, 1, 1, 7, 6, 1, 4, 8, 1, 1, 6, 2, 5, 7, 8,
            2, 8, 5, 6, 1, 4, 4, 7, 2, 6, 3, 6, 7,
        ];

        const WIDTH: usize = 10;

        let steps = loop_until_all_flash(data, WIDTH);
        assert_eq!(steps, 348);
    }
}
