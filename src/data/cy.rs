//! Cyprus
use super::*;

/// Generate holiday map for Cyprus.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2000, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2000, 3, 13)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2000, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2000, 4, 28)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2000, 4, 30)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2000, 5, 1)?,
                "Δευτέρα της Διακαινησίμου; Πρωτομαγιά",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 19)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2000, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2000, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2000, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2000, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2001, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2001, 2, 26)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2001, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2001, 4, 16)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2001, 6, 4)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2001, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2001, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2001, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2001, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2002, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2002, 3, 18)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2002, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2002, 5, 3)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2002, 5, 5)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2002, 5, 6)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2002, 6, 24)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2002, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2002, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2002, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2002, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2003, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2003, 3, 10)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2003, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2003, 4, 25)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2003, 4, 27)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2003, 4, 28)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2003, 6, 16)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2003, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2003, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2003, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2003, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2004, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2004, 2, 23)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2004, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2004, 4, 12)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2004, 5, 31)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2004, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2004, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2004, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2004, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2005, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2005, 3, 14)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2005, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2005, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2005, 4, 29)?, "Μεγάλη Παρασκευή"),
            (
                NaiveDate::from_ymd_res(2005, 5, 1)?,
                "Κυριακή του Πάσχα; Πρωτομαγιά",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 2)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (
                NaiveDate::from_ymd_res(2005, 6, 20)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2005, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2005, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2005, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2005, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2006, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2006, 3, 6)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2006, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2006, 4, 21)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2006, 4, 23)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2006, 4, 24)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2006, 6, 12)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2006, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2006, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2006, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2007, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2007, 2, 19)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2007, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2007, 4, 9)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2007, 5, 28)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2007, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2007, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2007, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2008, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2008, 3, 10)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2008, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2008, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2008, 4, 25)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2008, 4, 27)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2008, 4, 28)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2008, 6, 16)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2008, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2008, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2008, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2009, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2009, 3, 2)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2009, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2009, 4, 17)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2009, 4, 19)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2009, 4, 20)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2009, 6, 8)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2009, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2009, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2009, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2009, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2010, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2010, 2, 15)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2010, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2010, 4, 5)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2010, 5, 24)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2010, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2010, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2010, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2010, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2011, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2011, 3, 7)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2011, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2011, 4, 25)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2011, 6, 13)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2011, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2011, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2011, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2012, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2012, 2, 27)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2012, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2012, 4, 13)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2012, 4, 15)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2012, 4, 16)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2012, 6, 4)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2012, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2012, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2013, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2013, 3, 18)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2013, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2013, 5, 3)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2013, 5, 5)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2013, 5, 6)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2013, 6, 24)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2013, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2013, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2014, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2014, 3, 3)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2014, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2014, 4, 21)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2014, 6, 9)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2014, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2014, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2014, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2015, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2015, 2, 23)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2015, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2015, 4, 10)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2015, 4, 12)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2015, 4, 13)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2015, 6, 1)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2015, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2015, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2015, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2015, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2016, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2016, 3, 14)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2016, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2016, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2016, 4, 29)?, "Μεγάλη Παρασκευή"),
            (
                NaiveDate::from_ymd_res(2016, 5, 1)?,
                "Κυριακή του Πάσχα; Πρωτομαγιά",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (
                NaiveDate::from_ymd_res(2016, 6, 20)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2016, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2016, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2016, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2017, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2017, 2, 27)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2017, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2017, 4, 17)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2017, 6, 5)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2017, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2017, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2017, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2018, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2018, 2, 19)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2018, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2018, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2018, 4, 6)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2018, 4, 8)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2018, 4, 9)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2018, 5, 28)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2018, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2018, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2019, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2019, 3, 11)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2019, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2019, 4, 26)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2019, 4, 28)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2019, 4, 29)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2019, 6, 17)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2019, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2019, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2020, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2020, 3, 2)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2020, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2020, 4, 17)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2020, 4, 19)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2020, 4, 20)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2020, 6, 8)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2020, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2020, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2021, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2021, 3, 15)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2021, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2021, 4, 30)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2021, 5, 2)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2021, 5, 3)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2021, 6, 21)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2021, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2021, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2021, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2022, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2022, 3, 7)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2022, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2022, 4, 22)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2022, 4, 24)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2022, 4, 25)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2022, 6, 13)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2022, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2022, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2022, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2023, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2023, 2, 27)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2023, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2023, 4, 14)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2023, 4, 16)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2023, 4, 17)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2023, 6, 5)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2023, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2023, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2023, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2024, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2024, 3, 18)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2024, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2024, 5, 3)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2024, 5, 5)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2024, 5, 6)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2024, 6, 24)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2024, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2024, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2024, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2025, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2025, 3, 3)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2025, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2025, 4, 20)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2025, 4, 21)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2025, 6, 9)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2025, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2025, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2025, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2025, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2026, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2026, 2, 23)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2026, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2026, 4, 10)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2026, 4, 12)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2026, 4, 13)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2026, 6, 1)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2026, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2026, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2026, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2027, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2027, 3, 15)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2027, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2027, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2027, 4, 30)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2027, 5, 2)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2027, 5, 3)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2027, 6, 21)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2027, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2027, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2027, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2027, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2028, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2028, 2, 28)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2028, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2028, 4, 16)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2028, 4, 17)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2028, 6, 5)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2028, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2028, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2029, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2029, 2, 19)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2029, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2029, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2029, 4, 6)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2029, 4, 8)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2029, 4, 9)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2029, 5, 28)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2029, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2029, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2029, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2029, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Πρωτοχρονιά"),
            (NaiveDate::from_ymd_res(2030, 1, 6)?, "Ημέρα των Θεοφανίων"),
            (NaiveDate::from_ymd_res(2030, 3, 11)?, "Καθαρά Δευτέρα"),
            (
                NaiveDate::from_ymd_res(2030, 3, 25)?,
                "Ημέρα της Ελληνικής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 1)?, "Εθνική Ημέρα Κύπρου"),
            (NaiveDate::from_ymd_res(2030, 4, 26)?, "Μεγάλη Παρασκευή"),
            (NaiveDate::from_ymd_res(2030, 4, 28)?, "Κυριακή του Πάσχα"),
            (
                NaiveDate::from_ymd_res(2030, 4, 29)?,
                "Δευτέρα της Διακαινησίμου",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Πρωτομαγιά"),
            (
                NaiveDate::from_ymd_res(2030, 6, 17)?,
                "Δευτέρα του Αγίου Πνεύματος",
            ),
            (
                NaiveDate::from_ymd_res(2030, 8, 15)?,
                "Κοίμηση της Θεοτόκου",
            ),
            (
                NaiveDate::from_ymd_res(2030, 10, 1)?,
                "Ημέρα της Κυπριακής Ανεξαρτησίας",
            ),
            (NaiveDate::from_ymd_res(2030, 10, 28)?, "Ημέρα του Όχι"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Χριστούγεννα"),
            (
                NaiveDate::from_ymd_res(2030, 12, 26)?,
                "Επομένη Χριστουγέννων",
            ),
        ],
        &mut map,
        Country::CY,
        "Cyprus",
    );

    Ok(map)
}
