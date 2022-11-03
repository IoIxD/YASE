pub mod decomp;
pub mod block;

fn main() {
    let project = decomp::Project::new(None);
    println!("{:#?}",project);
}
