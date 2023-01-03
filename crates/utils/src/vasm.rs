use rand::distributions::{Alphanumeric, DistString};
use std::env::temp_dir;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn compile(assembly: &[u8]) -> Vec<u8> {
    let file_name: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let tmpdir = temp_dir();
    let vasm_path = PathBuf::from(env!("OUT_DIR")).join("vasm6502_oldstyle");
    let out_path = tmpdir.join(file_name);
    let mut child = Command::new(vasm_path)
        .arg("-Fbin")
        .arg("-dotdir")
        .arg("-o")
        .arg(&out_path)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Could not spawn vasm6502_oldstyle");

    let child_stdin = child.stdin.as_mut().unwrap();
    child_stdin.write_all(assembly).unwrap();

    let res = child.wait_with_output().unwrap();

    if !res.status.success() {
        panic!("Compile error")
    }

    let res = std::fs::read(&out_path).unwrap();
    std::fs::remove_file(&out_path).unwrap();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let assembly = b"  LDA #01";
        let res = compile(&assembly[..]);
        assert_eq!(res, vec![0xA9, 1])
    }
}
