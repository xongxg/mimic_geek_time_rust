fn main() {
    let env = "PATH=/usr/bin".to_string();

    let cmd = "cat /etc/passwd";
    let r1 = execute(cmd, BaseExecutor { env });
    println!("{:?}", r1);

    // let r2 = execute(cmd, |cmd: &str| {
    //     Ok(format!("fake fish execute: env: {}, cmd: {}", env, cmd))
    // });
    // println!("{:?}", r2);
}

trait Executor {
    fn execute(&self, cmd: &str) -> Result<String, &'static str>;
}

struct BaseExecutor {
    env: String,
}

impl Executor for BaseExecutor {
    fn execute(&self, cmd: &str) -> Result<String, &'static str> {
        Ok(format!(
            "fake bash execute: env: {}, cmd: {}",
            cmd, self.env
        ))
    }
}

fn execute(cmd: &str, exec: impl Executor) -> Result<String, &'static str> {
    exec.execute(cmd)
}
