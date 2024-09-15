use std::collections::HashSet;
use std::io::Write;
use pad::{p, Position, PositionPrinter};
use shadowcasting::shadow_cast;

fn main() {
    let stdin = std::io::stdin();
    let mut buffer = String::new();

    let mut current_position = p!(0, 0);
    let radius = 12;
    let positions = p!(-15, -15).iter_to(p!(15, 15)).collect::<HashSet<_>>();
    let walls = [
        p!(5, 5),
        p!(6, 5),
    ];

    let position_in_visible_area = |pos: Position| positions.contains(&pos);
    let position_blocks_view = move |pos: Position| walls.contains(&pos);

    loop {
        clear_screen();
        buffer.clear();

        let los = shadow_cast(current_position, radius, position_in_visible_area, position_blocks_view);

        std::io::stdout().flush().unwrap();

        PositionPrinter::new()
            .draw_axis(false)
            .position_mapping(move |pos, _| if pos == current_position {
                'P'
            } else if los.contains(&pos) && walls.contains(&pos) {
                'V'
            } else if los.contains(&pos) {
                ' '
            } else if walls.contains(&pos) {
                'W'
            } else {
                '#'
            }).print(positions.iter().cloned());

        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();

        let input = buffer.trim();

        match input {
            "w" => current_position = current_position + p!(0, 1),
            "s" => current_position = current_position + p!(0, -1),
            "a" => current_position = current_position + p!(- 1, 0),
            "d" => current_position = current_position + p!(1, 0),
            "exit" => break,
            _ => {}
        }
    }
}

fn clear_screen() {
    // Escape sequence to clear the screen and move the cursor to the top-left
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    std::io::stdout().flush().unwrap();
}

