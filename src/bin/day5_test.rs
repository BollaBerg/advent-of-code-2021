use super::*;

#[test]
fn vent_reads_from_string_correctly() {
    let vent = Vent::from_string("0,9 -> 5,9");
    assert_eq!(vent, Vent {start : (0, 9), end : (5, 9)});

    let vent_2 = Vent::from_string("19,247 -> 999,0");
    assert_eq!(vent_2, Vent {start : (19, 247), end : (999, 0)});
}

#[test]
fn vent_updates_horizontal_correctly() {
    let vent = Vent::from_string("0,9 -> 5,9");
    let mut array : [[u8; 1000] ; 1000] = [[0; 1000] ; 1000];

    vent.update_only_horizontal_and_vertical(&mut array);

    let mut manual_array : [[u8; 1000] ; 1000] = [[0; 1000] ; 1000];
    for x in 0..=5 {
        manual_array[x][9] = 1;
    }

    assert_eq!(array, manual_array);
}

#[test]
fn vent_updates_vertical_correctly() {
    let vent = Vent::from_string("5,0 -> 5,9");
    let mut array : [[u8; 1000] ; 1000] = [[0; 1000] ; 1000];

    vent.update_only_horizontal_and_vertical(&mut array);

    let mut manual_array : [[u8; 1000] ; 1000] = [[0; 1000] ; 1000];
    for y in 0..=9 {
        manual_array[5][y] = 1;
    }

    assert_eq!(array, manual_array);
}

#[test]
fn vent_does_not_update_non_horizontal_vents() {
    let vent = Vent::from_string("0,9 -> 5,0");
    let mut array : [[u8; 1000] ; 1000] = [[0; 1000] ; 1000];
    vent.update_only_horizontal_and_vertical(&mut array);

    assert_eq!(array, [[0; 1000] ; 1000]);
}

#[test]
fn count_counts_correctly() {
    let mut map : [[u8; 1000] ; 1000] = [[0; 1000] ; 1000];
    map[0][0] = 2;
    map[19][23] = 9;
    map[22][31] = 100;
    map[1][1] = 1;
    map[99][99] = 1;
    assert_eq!(count(&map), 3);
}

#[test]
fn test_total_task_1() {
    let inputs = 
        "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    let mut map : [[u8; 1000] ; 1000] = [[0; 1000] ; 1000];
    for line in inputs.lines() {
        let vent = Vent::from_string(line);
        vent.update_only_horizontal_and_vertical(&mut map);
    }

    assert_eq!(count(&map), 5);
}

#[test]
fn vent_updates_diagonal_correctly() {
    let vent = Vent::from_string("1,1 -> 7,7");
    let mut array : [[u8; 1000] ; 1000] = [[0; 1000] ; 1000];

    vent.update_all(&mut array);

    let mut manual_array : [[u8; 1000] ; 1000] = [[0; 1000] ; 1000];
    for i in 1..=7 {
        manual_array[i][i] = 1;
    }

    assert_eq!(array, manual_array);
}

#[test]
fn test_total_task_2() {
    let inputs = 
        "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    let mut map : [[u8; 1000] ; 1000] = [[0; 1000] ; 1000];
    for line in inputs.lines() {
        let vent = Vent::from_string(line);
        vent.update_all(&mut map);
    }

    assert_eq!(count(&map), 12);
}