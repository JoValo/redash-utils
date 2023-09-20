use super::*;

pub struct Vacuum {
}

impl Vacuum {
    pub fn execute(table: String) {
        let cmd = format!("VACUUM FULL {};", table);
        let output = Command::new("docker")
            .args(&["exec", "-it", "redash_postgres_1", "psql", "-U", "postgres", "-d", "postgres", "-c", &cmd])
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("Successful vacuum table: {}.", table);
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("{}", stderr)
                }
            }
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }
}