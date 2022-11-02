pub mod decomp;

fn main() {
    let project = decomp::Project::new(None);
    println!("{:#?}",project);
}
