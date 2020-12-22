use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

struct TimetableNotes {
    earliest_departure: i64,
    services: Vec<Option<i64>>
}

struct Departure {
    service_id: i64,
    departure_time: i64,
}

impl TimetableNotes {
    fn load(f: File) -> TimetableNotes {
        let mut lines = BufReader::new(f).lines();
        let first = lines.next().unwrap().unwrap();
        let second = lines.next().unwrap().unwrap();

        TimetableNotes{
            earliest_departure: first.parse().unwrap(),
            services: second.split(",").map(|v| match v {
                "x" => None,
                _ => Some(v.parse().unwrap())
            }).collect(),
        }
    }

    fn earliest_departure_service(&self) -> Departure {
        let departures = self.services.iter().filter_map(|x| {
            match x {
                None => None,
                Some(service_id) => {
                    let remainder = self.earliest_departure % service_id;
                    let departure_time = if remainder == 0 {
                        self.earliest_departure
                    } else {
                        self.earliest_departure + (service_id - remainder)
                    };
                    Some(Departure{
                        service_id: *service_id,
                        departure_time,
                    })
                },
            }
        });

        departures.min_by(|x, y| x.departure_time.cmp(&y.departure_time)).unwrap()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Please specify a file");
    }
    let f = File::open(args.get(1).unwrap()).unwrap();
    let notes = TimetableNotes::load(f);

    let d = notes.earliest_departure_service();
    let wait_time = d.departure_time - notes.earliest_departure;
    println!("Part 1: service {} x {} mins = {}", d.service_id, wait_time, d.service_id * wait_time);

    println!("Part 2: earliest consecutive departures: {}", earliest_consecutive(&notes.services, 100000000000000));
}

fn earliest_consecutive(service_ids: &Vec<Option<i64>>, min_timestamp: i64) -> i64 {
    let mut step_idx = 0;
    let mut step = service_ids[0].unwrap();
    let mut start = min_timestamp + step - (min_timestamp % step);
    println!("Starting from {}, step {}", start, step);
    loop {
        let mut found = true;
        'check_services: for (i, service) in service_ids.iter().enumerate() {
            if let Some(service_id) = service {
                let is_ok = (start + i as i64) % service_id == 0;
                if !is_ok {
                    found = false;
                    break 'check_services
                } else if i > step_idx {
                    step_idx = i;
                    step *= service_id;
                }
            } 
        }
        if found {
            return start;
        }
        start += step;
    }
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use crate::{TimetableNotes, earliest_consecutive};

    #[test]
    fn test_load_notes() {
        let f = File::open("sample.txt").unwrap();
        let notes = TimetableNotes::load(f);

        assert_eq!(939, notes.earliest_departure);
        assert_eq!(Some(7), notes.services[0]);
        assert_eq!(Some(13), notes.services[1]);
        assert_eq!(None, notes.services[2]);
        assert_eq!(None, notes.services[3]);
        assert_eq!(Some(59), notes.services[4]);
        assert_eq!(None, notes.services[5]);
        assert_eq!(Some(31), notes.services[6]);
        assert_eq!(Some(19), notes.services[7]);
    }

    #[test]
    fn test_earliest_departure() {
        let f = File::open("sample.txt").unwrap();
        let notes = TimetableNotes::load(f);

        let d = notes.earliest_departure_service();
        assert_eq!(59, d.service_id);
        assert_eq!(944, d.departure_time);
    }

    #[test]
    fn test_find_consecutive_departures() {
        let list1 = vec![Some(17), None, Some(13), Some(19)];
        assert_eq!(3417, earliest_consecutive(&list1, 0));
        let list2 = vec![Some(67),Some(7),Some(59),Some(61)];
        assert_eq!(754018, earliest_consecutive(&list2, 0));
        let list3 = vec![Some(67),None,Some(7),Some(59),Some(61)];
        assert_eq!(779210, earliest_consecutive(&list3, 0));
        let list4 = vec![Some(67),Some(7),None,Some(59),Some(61)];
        assert_eq!(1261476, earliest_consecutive(&list4, 0));
        let list5 = vec![Some(1789),Some(37),Some(47),Some(1889)];
        assert_eq!(1202161486, earliest_consecutive(&list5, 0));
    }

    #[test]
    fn test_find_consecutive_departures_sample() {
        let f = File::open("sample.txt").unwrap();
        let notes = TimetableNotes::load(f);

        assert_eq!(1068781, earliest_consecutive(&notes.services, 0));
        assert_eq!(1068781, earliest_consecutive(&notes.services, 3));
    }
}
