use std::{collections::{HashMap, HashSet}, f32};

use draw::Canvas;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(visualize_area_computation(input));
}

fn visualize_area_computation(input: &str) {
    let mut neighbours : HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    neighbours.insert('S', vec![(-1, 0), (0, -1), (0, 1), (1, 0)]);
    neighbours.insert('|', vec![(0, -1), (0, 1)]);
    neighbours.insert('-', vec![(-1, 0), (1, 0)]);
    neighbours.insert('L', vec![(0, -1), (1, 0)]);
    neighbours.insert('J', vec![(-1, 0), (0, -1)]);
    neighbours.insert('7', vec![(-1, 0), (0, 1)]);
    neighbours.insert('F', vec![(0, 1), (1, 0)]);

    let empty : Vec<(i32, i32)> = vec![];

    let mut start : (i32, i32) = (-1, -1);

    let lines = input.lines();
    let board = lines.enumerate().map(|(y, l)| {
        l.chars().enumerate().map(|(x, c)| {
            let n = neighbours.get(&c).unwrap_or(&empty);
            if n.len() == 4 {
                start = (x as i32, y as i32);
            }
            n
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut path : Vec<(i32, i32)> = vec![(0, 0), (0, 0)];

    for i in 0..4 {
        let mut move_to = neighbours.get(&'S').unwrap()[i];
        let mut next_tile = (start.0 + move_to.0, start.1 + move_to.1);
        let mut visited : HashSet<(i32, i32)> = HashSet::new();
        visited.insert(start);
        let mut result_loop : Vec<(i32, i32)> = vec![start];
        while visited.insert(next_tile) {
            // new tile!
            result_loop.push(next_tile);
            let neighbours = board[next_tile.1 as usize][next_tile.0 as usize];
            let neighbours = neighbours.iter().filter(|(x, y)| {
                !(move_to.0 == -x && move_to.1 == -y)
            }).collect::<Vec<_>>();
            if neighbours.len() != 1
            {
                break;
            }
            move_to = *neighbours[0];
            next_tile = (next_tile.0 + move_to.0, next_tile.1 + move_to.1);
        }
        if next_tile == start {
            path = result_loop;
            path.push(start);
            break;
        }
    }

    let mut canvas = Canvas::new(140, 140);
    canvas.display_list.add(draw::Drawing::new()
        .with_shape(draw::Shape::Rectangle {
            width: 140,
            height: 140,
        })
        .with_xy(0.0, 0.0)
        .with_style(draw::Style::filled(draw::RGB { r:255, g:255, b:255 }))
        );

    //let mut i = 0;
    //for p in &path {
    //    canvas.display_list.add(
    //        draw::Drawing::new()
    //        .with_shape(draw::Shape::Rectangle {
    //            width: 1,
    //            height: 1,
    //        })
    //        .with_xy(p.0 as f32, p.1 as f32)
    //        .with_style(draw::Style::filled(draw::RGB {r:255, g:0, b:0})));
    //    draw::render::save(
    //        &canvas,
    //        format!("tests/svg/path/path_{:05}.svg", i).as_str(),
    //        draw::SvgRenderer::new(),
    //        )
    //        .expect("Failed to save");
    //    i += 1;
    //}

    // now draw the area

    let mut my_drawings = vec![];
    for w in path.windows(2) {
        if w[0].0 == w[1].0 {
            continue;
        }
        let col = if w[0].0 < w[1].0 {
            draw::RGB { r: 255, g: 0, b: 0 }
        }
        else {
            draw::RGB { r: 255, g: 255, b: 255 }
        };
        my_drawings.push((std::cmp::max(w[0].1, w[1].1),
            draw::Drawing::new()
            .with_shape(draw::Shape::Rectangle {
                width: 1,
                height: std::cmp::max(w[0].1, w[1].1) as u32,
            })
            .with_xy(std::cmp::min(w[0].0, w[1].0) as f32, 0.0)
            .with_style(draw::Style::filled(col))
            ));
    }
    my_drawings.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap().reverse());
    let mut i = 0;
    for d in my_drawings {
        canvas.display_list.add(d.1);
        draw::render::save(
            &canvas,
            format!("tests/svg/area/area_{:05}.svg", i).as_str(),
            draw::SvgRenderer::new(),
            )
            .expect("Failed to save");
        i += 1;
    }

    for p in &path {
        canvas.display_list.add(
            draw::Drawing::new()
            .with_shape(draw::Shape::Rectangle {
                width: 1,
                height: 1,
            })
            .with_xy(p.0 as f32, p.1 as f32)
            .with_style(draw::Style::filled(draw::RGB {r:0, g:0, b:255})));
        draw::render::save(
            &canvas,
            format!("tests/svg/area/area_{:05}.svg", i).as_str(),
            draw::SvgRenderer::new(),
            )
            .expect("Failed to save");
        i += 1;
    }

    draw::render::save(
        &canvas,
        format!("tests/svg/result_area.svg").as_str(),
        draw::SvgRenderer::new(),
        )
        .expect("Failed to save");
    //draw::render::save(
    //    &canvas,
    //    "tests/svg/basic_end_to_end.svg",
    //    renderer,
    //    )
    //    .expect("Failed to save")

}
