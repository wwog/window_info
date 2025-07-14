mod platforms;

fn main() {
    let windows = platforms::list_windows();
    println!("Found {} windows:", windows.len());
    for (i, window) in windows.iter().enumerate() {
        println!("  {}: {}", i + 1, window);
    }
}
