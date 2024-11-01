use std::cmp::{max, min};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

fn area_occupied(rectangles: &Vec<Rectangle>) -> i32 {
    let mut occupied_area = 0;
    let mut events = vec![];

    for rect in rectangles {
        let x1 = min(rect.a.x, rect.b.x);
        let x2 = max(rect.a.x, rect.b.x);
        let y1 = min(rect.a.y, rect.b.y);
        let y2 = max(rect.a.y, rect.b.y);

        events.push((x1, y1, y2, 1)); 
        events.push((x2, y1, y2, -1)); 
    }

    events.sort();

    let mut active_intervals = vec![];
    let mut prev_x = events[0].0;

    for (x, y1, y2, typ) in events {
        let mut height = 0;
        let mut last_y = 0;
        for &(y_start, y_end) in &active_intervals {
            height += max(0, y_end - max(y_start, last_y));
            last_y = max(last_y, y_end);
        }

        occupied_area += height * (x - prev_x);
        prev_x = x;

        if typ == 1 {
            active_intervals.push((y1, y2));
        } else {
            active_intervals.retain(|&(a, b)| !(a == y1 && b == y2));
        }
        active_intervals.sort();
    }

    occupied_area
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle { a: Point { x: 2, y: 9 }, b: Point { x: 5, y: 3 } },
        Rectangle { a: Point { x: 1, y: 8 }, b: Point { x: 11, y: 6 } },
        Rectangle { a: Point { x: 9, y: 10 }, b: Point { x: 13, y: 2 } },
    ]
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
    println!("Test passed! Occupied area: {}", occupied);
}

fn main() {
    area_occupied_test();
}
