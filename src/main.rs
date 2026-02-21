use rdev::{Event, EventType, listen};

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(key) => {
            println!("Key pressed: {:?}", key);
            // คุณสามารถเพิ่ม Logic การบันทึกลงไฟล์ (Logging) ตรงนี้ได้
        }
        EventType::KeyRelease(key) => {
            // println!("Key released: {:?}", key);
        }
        _ => (),
    }
}

fn main() {
    println!("Starting keyboard listener... Press Ctrl+C to stop.");

    // เริ่มต้นการดักฟัง Event
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}
