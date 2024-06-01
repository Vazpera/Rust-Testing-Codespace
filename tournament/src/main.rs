struct Name {
    first: String,
    last: String,
}
struct Date {
    day: u8,
    month: u8,
}
struct Person {
    name: Name,
    age: u8,
    dob: Date,
}

impl Person {
    fn to_string(self) -> String {
        return format!(
            "{} {}, aged {}, DOB: {}, {}",
            self.name.first,
            self.name.last,
            self.age,
            match self.dob.month {
                1 => "Jan",
                2 => "Feb",
                3 => "Mar",
                4 => "Apr",
                5 => "May",
                6 => "Jun",
                7 => "Jul",
                8 => "Aug",
                9 => "Sep",
                10 => "Oct",
                11 => "Nov",
                12 => "Dec",
                _ => "N/A",
            },
            self.dob.day
        );
    }
}

fn new_person(
    first_name: String,
    last_name: String,
    age: u8,
    birth_month: u8,
    birth_day: u8,
) -> Person {
    return Person {
        name: Name {
            first: first_name,
            last: last_name,
        },
        age,
        dob: Date {
            day: birth_day,
            month: birth_month,
        },
    };
}

fn main() {
    let p = new_person("John".to_string(), "Doe".to_string(), 32, 10, 8);
    println!("Hello, {}", p.to_string())
}
