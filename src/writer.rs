pub fn write<W: ?Sized>(
    w: &mut W,
    buffer: &Vec<crate::StringerResult>,
) -> Result<(), crate::error::StringerError>
where
    W: std::io::Write
{
    for r in buffer {
        match w.write(serde_json::to_string(r).unwrap().as_bytes()) {
            Err(_) => {
                println!("unable to write to buffer");
                return Err(crate::error::StringerError::new(String::from(
                    "unable to write to buffer",
                )));
            }
            _ => {
            }
        }
        w.write_fmt(format_args!("\n")).unwrap();
    }

    Ok(())
}
