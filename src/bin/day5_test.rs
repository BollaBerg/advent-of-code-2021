use super::*;

#[test]
fn vent_reads_from_string_correctly() {
    let vent = Vent::from_string("0,9 -> 5,9");
    assert_eq!(vent, Vent {start : (0, 9), end : (5, 9)});

    let vent_2 = Vent::from_string("19,247 -> 999,0");
    assert_eq!(vent_2, Vent {start : (19, 247), end : (999, 0)});
}
