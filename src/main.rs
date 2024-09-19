mod kent_schema;
mod sums_pluto_schema;

use boa_engine::{js_str, js_string};
use chrono::{DateTime, NaiveDateTime, Utc};
use icalendar::{Calendar, Component, EventLike, EventStatus};
use reqwest::Url;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // let calendar = kent_calendar("https://student.kent.ac.uk/events").await?;
    // calendar.print()?;
    let calendar = sums_calendar(
        "UutZYcRjdM5RzX2mnC8zPR",
        "Kent SU Calendar",
        "Hello Kent",
        |e| format!("https://hellokent.co.uk/events/id/{}", e.id),
    )
    .await?;
    calendar.print()?;
    Ok(())
}

async fn sums_calendar<T: Fn(&sums_pluto_schema::Event) -> String>(
    site_id: &str,
    title: &str,
    description: &str,
    url_formatter: T,
) -> Result<Calendar, anyhow::Error> {
    let mut calendar = icalendar::Calendar::new();
    calendar
        .name(title)
        .description(description)
        .timezone("Europe/London");

    let mut url = Url::parse_with_params(
        "https://pluto.sums.su/api/events",
        &[
            ("perPage", "4"),
            ("sortBy", "start_date"),
            ("futureOrOngoing", "0"),
            ("onlyPremium", "1"),
        ],
    )?;

    let client = reqwest::Client::new();
    loop {
        let response = dbg!(
            client
                .get(url.clone())
                .header("X-Site-Id", site_id)
                .send()
                .await?
        )
        .json::<sums_pluto_schema::Page>()
        .await?;
        eprintln!("{:?}", response);
        for event in response.data {
            let mut cal_event = icalendar::Event::new();
            cal_event
                .summary(&event.title)
                .description(&event.description)
                .starts(
                    DateTime::parse_from_rfc3339(&event.start_date)
                        .unwrap()
                        .with_timezone(&Utc),
                )
                .ends(
                    DateTime::parse_from_rfc3339(&event.end_date)
                        .unwrap()
                        .with_timezone(&Utc),
                )
                .url(&url_formatter(&event))
                .uid(&event.id.to_string());

            if let Some(venue) = event.venue {
                cal_event.location(&venue.name);
            }
            calendar.push(cal_event.done());
        }
        match response.next_page_url {
            Some(next_url) => url = next_url.parse()?,
            None => break,
        }
    }

    Ok(calendar)
}

async fn kent_calendar(url: &str) -> Result<Calendar, anyhow::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    let mut context = boa_engine::Context::default();

    let document = scraper::Html::parse_document(&body);
    let description_selector =
        scraper::Selector::parse("head > meta[name=\"description\"]").unwrap();
    let title_selector = scraper::Selector::parse("head > title").unwrap();
    let description = document
        .select(&description_selector)
        .next()
        .unwrap()
        .value()
        .attr("content")
        .unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .unwrap()
        .text()
        .collect::<String>();

    let mut calendar = icalendar::Calendar::new();
    calendar
        .name(&title)
        .description(description)
        .timezone("Europe/London");

    let script_selector = scraper::Selector::parse("script").unwrap();
    let script_elements = document.select(&script_selector);
    context
        .global_object()
        .set(
            js_string!("window"),
            context.global_object(),
            false,
            &mut context,
        )
        .unwrap();

    for element in script_elements {
        // println!("{:?}", element);
        // println!("{}", element.text().collect::<String>());
        let _ = context
            .eval(boa_engine::Source::from_bytes(
                &element.text().collect::<String>(),
            ))
            .inspect_err(|e| {
                eprintln!("{}", element.text().collect::<String>());
                eprintln!("Uncaught {e}");
            });
    }
    let data: kent_schema::Data = serde_json::from_value(
        context
            .global_object()
            .get(js_str!("KENT"), &mut context)
            .unwrap()
            .to_json(&mut context)
            .unwrap(),
    )?;

    eprintln!("{:?}", data);

    for event in data.events {
        let mut cal_event = icalendar::Event::new();
        cal_event
            .summary(&event.title)
            .description(&event.description)
            .starts(NaiveDateTime::parse_from_str(&event.start, "%Y-%m-%d %H:%M:%S").unwrap())
            .ends(NaiveDateTime::parse_from_str(&event.end, "%Y-%m-%d %H:%M:%S").unwrap())
            .location(&event.location)
            // .url(&event.url)
            .url(
                &(data.events_base_url.to_owned()
                    + "/"
                    + &event.id.to_string()
                    + "/"
                    + &event.slug),
            )
            .uid(&event.id.to_string());
        // .all_day(event.all_day)
        if event.tentative {
            cal_event.status(EventStatus::Tentative);
        }

        calendar.push(cal_event.done());
    }

    Ok(calendar)
}
