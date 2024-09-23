mod kent_schema;
mod sums_pluto_schema;
mod calendars;


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // let calendar = kent_calendar("https://student.kent.ac.uk/events").await?;
    // calendar.print()?;
    let calendar = calendars::sums_calendar(
        "UutZYcRjdM5RzX2mnC8zPR",
        "Kent SU Calendar",
        "Hello Kent",
        |e| format!("https://hellokent.co.uk/events/id/{}", e.id),
    )
    .await?;
    calendar.print()?;
    Ok(())
}

