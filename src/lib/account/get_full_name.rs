use crate::lib::database::models::PersonName;
use crate::lib::titlecase::string_to_title;

pub enum FullNameFormat {
    FirstLast,
    LastFirst,
    FirstMiddleLast,
    LastFirstMiddle,
    FirstMiddleInitialLast,
    PrefixFirstMiddleLast,
    LastPrefixFirstMiddle,
    FirstMiddleLastSuffix,
    LastFirstMiddleSuffix,
    PrefixFirstMiddleLastSuffix,
}

pub fn full_name_from_person(person: &PersonName, format: FullNameFormat, titlecase: bool) -> String {
    let last_name = &person.last_name;
    let first_name = &person.first_name;
    let middle_initial = match &person.middle_initial {
        Some(x) => {
            let x = x.to_string().chars().next();
            let x = match x {
                Some(x) => x.to_string(),
                None => "".to_string()
            };
            x
        }
        None => "".to_string()
    };

    let suffix = match &person.name_suffix {
        Some(x) => x.to_string(),
        None => "".to_string()
    };

    let prefix = match &person.name_prefix {
        Some(x) => x.to_string(),
        None => "".to_string()
    };

    let mut name = String::new();

    // suffix first because it's more likely to be empty
    name = match format {
        FullNameFormat::FirstLast => format!("{} {}", first_name, last_name),
        FullNameFormat::LastFirst => format!("{}, {}", last_name, first_name),
        FullNameFormat::FirstMiddleLast => format!("{} {} {}", first_name, middle_initial, last_name),
        FullNameFormat::LastFirstMiddle => format!("{}, {} {}", last_name, first_name, middle_initial),
        FullNameFormat::FirstMiddleInitialLast => format!("{} {} {}", first_name, middle_initial, last_name),
        FullNameFormat::PrefixFirstMiddleLast => format!("{} {} {} {}", prefix, first_name, middle_initial, last_name),
        FullNameFormat::LastPrefixFirstMiddle => format!("{}, {} {} {}", last_name, prefix, first_name, middle_initial),
        FullNameFormat::FirstMiddleLastSuffix => format!("{} {} {} {}", first_name, middle_initial, last_name, suffix),
        FullNameFormat::LastFirstMiddleSuffix => format!("{}, {} {} {}", last_name, first_name, middle_initial, suffix),
        FullNameFormat::PrefixFirstMiddleLastSuffix => format!("{} {} {} {} {}", prefix, first_name, middle_initial, last_name, suffix),
        // _ => format!("{} {} {}", first_name, middle_initial, last_name)
    };

    if titlecase {
        name = string_to_title(&*name);
    } else {
        name = name.to_uppercase();
    }

    name.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_name_from_person() {
        let person = PersonName {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            middle_initial: Some("Q".to_string()),
            name_prefix: Some("Mr".to_string()),
            name_suffix: Some("III".to_string()),
        };

        assert_eq!(full_name_from_person(&person, FullNameFormat::FirstLast, false), "JOHN DOE");
        assert_eq!(full_name_from_person(&person, FullNameFormat::LastFirst, false), "DOE, JOHN");
        assert_eq!(full_name_from_person(&person, FullNameFormat::FirstMiddleLast, false), "JOHN Q DOE");
        assert_eq!(full_name_from_person(&person, FullNameFormat::LastFirstMiddle, false), "DOE, JOHN Q");
        assert_eq!(full_name_from_person(&person, FullNameFormat::FirstMiddleInitialLast, false), "JOHN Q DOE");
        assert_eq!(full_name_from_person(&person, FullNameFormat::PrefixFirstMiddleLast, false), "MR JOHN Q DOE");
        assert_eq!(full_name_from_person(&person, FullNameFormat::LastPrefixFirstMiddle, false), "DOE, MR JOHN Q");
        assert_eq!(full_name_from_person(&person, FullNameFormat::FirstMiddleLastSuffix, false), "JOHN Q DOE III");
        assert_eq!(full_name_from_person(&person, FullNameFormat::LastFirstMiddleSuffix, false), "DOE, JOHN Q III");
        assert_eq!(full_name_from_person(&person, FullNameFormat::PrefixFirstMiddleLastSuffix, false), "MR JOHN Q DOE III");

        // titlecase
        assert_eq!(full_name_from_person(&person, FullNameFormat::FirstLast, true), "John Doe");
    }
}
