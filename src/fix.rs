use utils::run;
use protogen::test;

pub fn do_fix(in_dir: &str, out_dir: &str) -> Result<(), String> {
    run(in_dir, out_dir, &fix)
}

fn fix(msgs: Vec<test::Dummy>) -> Result<(Vec<test::Dummy>, usize), String> {
    Ok((msgs, 0))
}
