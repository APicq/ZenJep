use crate::flightlistgenerator::flighttime::FlightTime;
use crate::flightlistgenerator::{flight::Flight, FlightList};
use askama::Template;

#[derive(Template, Debug, Default)]
#[template(path = "jeppesen.html")]
pub struct JeppesenBookShelfHtml {
    bookshelf: Vec<JeppesenBook>,
}

impl JeppesenBookShelfHtml {
    pub fn build_from(flightlist: &FlightList) -> Self {
        let mut bookshelf = Vec::new();
        let mut jeppesen_book = JeppesenBook::default();
        let mut jeppesen_page = JeppesenPage::default();
        // accumulators
        let _total_this_page_current = TotalLine::ZERO;
        //let mut total_this_book_current = TotalLine::ZERO;
        let mut total_overall_current = TotalLine::ZERO;

        // marker
        // true because we consider new book
        //let mut new_book_found = true;

        let index_last_flight = flightlist.flights.len() - 1;
        for (index, flight) in flightlist.flights.iter().enumerate() {
            jeppesen_page.add_flight(flight);
            jeppesen_page.total_this_page.add_flight(flight);
            jeppesen_page.total_this_book.add_flight(flight);
            jeppesen_page.total_overall.add_flight(flight);

            // We have reached an end of page
            if flight.end_of_page || index == index_last_flight {
                // push the page into the book
                jeppesen_book.add_page(jeppesen_page.clone());
                // we save the total_current_page:
                //total_this_page_current = jeppesen_page.total_this_page;
                // we save the total_current_page:
                total_overall_current = jeppesen_page.total_overall;
                //total_this_book_current = jeppesen_page.total_this_book;

                // clear the page
                jeppesen_page.clear();
                jeppesen_page.total_overall = total_overall_current;

                if flight.end_of_book || index == index_last_flight {
                    bookshelf.push(jeppesen_book.clone());
                    jeppesen_book.clear();
                } else {
                    jeppesen_page.total_from_previous_pages = total_overall_current;
                }
            }
        }
        JeppesenBookShelfHtml { bookshelf }
    }
}

#[derive(Debug, Default, Clone)]
pub struct JeppesenBook {
    pages: Vec<JeppesenPage>,
}

impl JeppesenBook {
    fn add_page(&mut self, page: JeppesenPage) {
        self.pages.push(page)
    }
    fn clear(&mut self) {
        self.pages.clear();
    }
}

#[derive(Debug, Default, Clone)]
struct JeppesenPage {
    flightlinelist: Vec<Flight>,
    total_this_page: TotalLine,
    total_from_previous_pages: TotalLine,
    total_this_book: TotalLine,
    total_overall: TotalLine,
    _page_number: usize,
    _book_number: usize, // total_final
}

impl JeppesenPage {
    fn add_flight(&mut self, flight: &Flight) {
        self.flightlinelist.push(flight.clone());
    }

    fn clear(&mut self) {
        self.flightlinelist.clear();
        self.total_this_page = TotalLine::ZERO;
    }
}

/// The line with the total at the end of a page
/// in the jeppesen logbook
#[derive(Debug, Default, Copy, Clone)]
struct TotalLine {
    multi_pilot_time: FlightTime,
    total_flight_time: FlightTime,
    takeoff_day: u32,
    takeoff_night: u32,
    landing_day: u32,
    landing_night: u32,
    operational_condition_time_ifr: FlightTime,
    operational_condition_time_night: FlightTime,
    pilot_in_command_time: FlightTime,
    copilot_time: FlightTime,
    dual_time: FlightTime,
    instructor_time: FlightTime,
    sim_total_time_of_session: FlightTime,
}

impl TotalLine {
    pub const ZERO: Self = TotalLine {
        multi_pilot_time: FlightTime::ZERO,
        total_flight_time: FlightTime::ZERO,
        takeoff_day: 0,
        takeoff_night: 0,
        landing_day: 0,
        landing_night: 0,
        operational_condition_time_ifr: FlightTime::ZERO,
        operational_condition_time_night: FlightTime::ZERO,
        pilot_in_command_time: FlightTime::ZERO,
        copilot_time: FlightTime::ZERO,
        dual_time: FlightTime::ZERO,
        instructor_time: FlightTime::ZERO,
        sim_total_time_of_session: FlightTime::ZERO,
    };

    fn add_flight(&mut self, flight: &Flight) {
        if !flight.is_sim {
            self.multi_pilot_time += flight.multi_pilot_time;
            self.total_flight_time += flight.total_flight_time;
            self.takeoff_day += flight.takeoff_day as u32;
            self.takeoff_night += flight.takeoff_night as u32;
            self.landing_day += flight.landing_day as u32;
            self.landing_night += flight.landing_night as u32;
            self.operational_condition_time_ifr += flight.operational_condition_time_ifr;
            self.operational_condition_time_night += flight.operational_condition_time_night;
            self.pilot_in_command_time += flight.pilot_in_command_time;
            self.copilot_time += flight.copilot_time;
            self.dual_time += flight.dual_time;
            self.instructor_time += flight.instructor_time;
        } else {
            self.sim_total_time_of_session += flight.sim_total_time_of_session;
        }
    }
}
