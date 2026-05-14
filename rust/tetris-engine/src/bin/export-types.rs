fn main() {
    tetris_engine::export_types().expect("failed to export TypeScript types");
    eprintln!("exported → rust-tetris/src/generated/engine-types.ts");
}
