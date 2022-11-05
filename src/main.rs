pub mod decomp;
pub mod blocks;
pub mod block_names;

fn main() {
    let project = decomp::Project::new(None);
    println!("{:#?}",project);
}
