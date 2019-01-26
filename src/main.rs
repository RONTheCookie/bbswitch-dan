use std::fs;

fn main() {
    let current = String::from_utf8(fs::read("/proc/acpi/bbswitch").unwrap()).unwrap().contains("ON");
    fs::write("/proc/acpi/bbswitch", if current {
        "OFF"
    } else {
        "ON"
    }).unwrap();
}
