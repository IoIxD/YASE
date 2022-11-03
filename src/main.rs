pub mod decomp;
pub mod blocks;

fn main() {
    let project = decomp::Project::new(None);
    println!("{:#?}",project);
}
