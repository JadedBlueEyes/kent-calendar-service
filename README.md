# Kent Calendar Service

## What is this?

This is a service that converts the events available on the University of Kent website and the Kent Student Union website into iCal format.

## How do I use it?

You can run the service locally

### From source

```sh
cargo run
```

### From docker

```sh
docker run -p 3779:3779 JadedBlueEyes/kent-calendar-service
```

Then visit <http://localhost:3779> and download the calendars you are interested in

### Why is there not a hosted version?

The Kent Student Union and Kent websites block access from cloud services. This means that the service cannot be hosted on a cloud service and must be run locally.

## How does it work?

### University of Kent website

The service first retrieves the page with the events from the University of Kent website.
It then parses the HTML to retrieve metadata and all of the script tags on the page.
The script tags are then evaluated in a sandboxed JavaScript environment, and the events are extracted from the global `KENT` object.
Finally, the events are converted into an iCal calendar and returned to the client.

### Kent Student Union website

The retrieves events from the SUMS Pluto API. The service enumerates all pages to retrieve all events. The events are then converted into an iCal calendar and returned to the client.
