use super::*;

#[test]
fn vent_reads_from_string_correctly() {
    let vent = Vent::from_string("0,9 -> 5,9");
    assert_eq!(vent, Vent {start : (0, 9), end : (5, 9)});

    let vent_2 = Vent::from_string("19,247 -> 999,0");
    assert_eq!(vent_2, Vent {start : (19, 247), end : (999, 0)});
}

#[test]
fn vent_updates_horizontal_and_vertical_correctly() {
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
fn vent_does_not_update_non_horizontal_vents() {
    let vent = Vent::from_string("0,9 -> 5,0");
    let mut array : [[u8; 1000] ; 1000] = [[0; 1000] ; 1000];
    vent.update_only_horizontal_and_vertical(&mut array);

    assert_eq!(array, [[0; 1000] ; 1000]);
}