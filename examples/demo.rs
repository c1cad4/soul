use cyb_soul::Memory;
fn main() {
    let m = Memory {
        id: "local-memory".into(),
        time: "2026-10-10".into(),
        text: "The farm observation was saved".into(),
        source: "demo-fixture".into(),
        importance: 0.8,
    };
    m.validate().unwrap();
    println!("{}: {}", m.id, m.text);
}
