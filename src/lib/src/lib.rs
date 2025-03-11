mod golang;

use std::io;

pub fn generate(_: impl io::Read) -> Result<String, String> {
    Ok("export interface Person {\n\tname: string\n\tage: number\n\tbirthday: Date\n}\n".to_string())
}
