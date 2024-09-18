mod schema;

use boa_engine::{js_str, js_string, NativeFunction};
use chrono::NaiveDateTime;
use icalendar::{Component, EventLike, EventStatus};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://student.kent.ac.uk/events").await?;
    let body = response.text().await?;
    
    let mut context = boa_engine::Context::default();

    let document = scraper::Html::parse_document(&body);
    let script_selector = scraper::Selector::parse("script").unwrap();
    let script_elements = document.select(&script_selector);
    context.global_object().set(js_string!("window"), context.global_object(), false, &mut context).unwrap();
    for element in script_elements.filter(|e|e.text().next().is_some()) {
        // println!("{:?}", element);
        // println!("{}", element.text().collect::<String>());
        match context.eval(boa_engine::Source::from_bytes(&element.text().collect::<String>())) {
            Ok(_) => {
                // let res_str = res.to_string(&mut context).unwrap().to_std_string_escaped();
                // println!(
                //     "{}",
                //     res_str
                // );
                // if res_str != "undefined" {
                // res.to_json(&mut context).ok().inspect(|json| println!("{}", json));
                // }
            }
            Err(e) => {
                // Pretty print the error
                eprintln!("{}", element.text().collect::<String>());
                eprintln!("Uncaught {e}");
            }
        };
    }
    let data: schema::Data = serde_json::from_value(context.global_object().get(js_str!("KENT"), &mut context).unwrap().to_json(&mut context).unwrap().try_into().unwrap())?;
    eprintln!("{:?}", data);
    let mut my_calendar = icalendar::Calendar::new();
    my_calendar
        .name("Kent Calendar")
        .description("Kent Calendar")
        .timezone("Europe/London");
    for event in data.events {

        let mut cal_event = icalendar::Event::new();
        cal_event
            .summary(&event.title)
            .description(&event.description)
            .starts(NaiveDateTime::parse_from_str(&event.start, "%Y-%m-%d %H:%M:%S").unwrap())
            .ends(NaiveDateTime::parse_from_str(&event.end, "%Y-%m-%d %H:%M:%S").unwrap())
            .location(&event.location)
            // .url(&event.url)
            .url(&(data.events_base_url.to_owned() + "/" + &event.id.to_string() + "/" + &event.slug))
            .uid(&event.id.to_string());
            // .all_day(event.all_day)
        if event.tentative {
            cal_event.status(EventStatus::Tentative);
        }
        
        my_calendar.push(cal_event.done());
    }
    my_calendar.print()?;
    

    Ok(())
}


