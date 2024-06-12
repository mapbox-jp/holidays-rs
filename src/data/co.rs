//! Colombia
use super::*;

/// Generate holiday map for Colombia.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2000, 1, 10)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 20)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 20)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2000, 6, 5)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 26)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2000, 7, 3)?,
                "Sagrado Corazón (observado); San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2000, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2000, 8, 21)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 10, 16)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 11, 6)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 11, 13)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2001, 1, 8)?,
                "Día de los Reyes Magos (observado)",
            ),
            (NaiveDate::from_ymd_res(2001, 3, 19)?, "Día de San José"),
            (NaiveDate::from_ymd_res(2001, 4, 12)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2001, 5, 28)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 6, 18)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2001, 6, 25)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 7, 2)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2001, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2001, 8, 20)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 10, 15)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 11, 5)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 11, 12)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2002, 1, 7)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 3, 25)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 28)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2002, 5, 13)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 6, 3)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2002, 6, 10)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 7, 1)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2002, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2002, 8, 19)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 10, 14)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 11, 4)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 11, 11)?,
                "Independencia de Cartagena",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2003, 1, 6)?,
                "Día de los Reyes Magos",
            ),
            (
                NaiveDate::from_ymd_res(2003, 3, 24)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 17)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2003, 6, 2)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 6, 23)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2003, 6, 30)?,
                "Sagrado Corazón (observado); San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2003, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2003, 8, 18)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 10, 13)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 3)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 17)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2004, 1, 12)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 3, 22)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 8)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2004, 5, 24)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 6, 14)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2004, 6, 21)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 7, 5)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2004, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2004, 8, 16)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 10, 18)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 15)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2005, 1, 10)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 3, 21)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 24)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2005, 5, 9)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 30)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2005, 6, 6)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 7, 4)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2005, 8, 7)?, "Batalla de Boyacá"),
            (NaiveDate::from_ymd_res(2005, 8, 15)?, "La Asunción"),
            (
                NaiveDate::from_ymd_res(2005, 10, 17)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 7)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 14)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2006, 1, 9)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 3, 20)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 13)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2006, 5, 29)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 6, 19)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2006, 6, 26)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 7, 3)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2006, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2006, 8, 21)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 16)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 11, 6)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 11, 13)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2007, 1, 8)?,
                "Día de los Reyes Magos (observado)",
            ),
            (NaiveDate::from_ymd_res(2007, 3, 19)?, "Día de San José"),
            (NaiveDate::from_ymd_res(2007, 4, 5)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2007, 5, 21)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 6, 11)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2007, 6, 18)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 7, 2)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2007, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2007, 8, 20)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 15)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 11, 5)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 11, 12)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2008, 1, 7)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 24)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 20)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2008, 5, 5)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 26)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2008, 6, 2)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 6, 30)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2008, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2008, 8, 18)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 13)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 11, 3)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 11, 17)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2009, 1, 12)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 23)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 9)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2009, 5, 25)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 6, 15)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2009, 6, 22)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2009, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2009, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2009, 8, 17)?,
                "La Asunción (observado)",
            ),
            (NaiveDate::from_ymd_res(2009, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2009, 11, 2)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 16)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2010, 1, 11)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 3, 22)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 1)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2010, 5, 17)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 6, 7)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2010, 6, 14)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 7, 5)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2010, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2010, 8, 16)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 10, 18)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 15)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2011, 1, 10)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 3, 21)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 21)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2011, 6, 6)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 6, 27)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2011, 7, 4)?,
                "Sagrado Corazón (observado); San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2011, 8, 7)?, "Batalla de Boyacá"),
            (NaiveDate::from_ymd_res(2011, 8, 15)?, "La Asunción"),
            (
                NaiveDate::from_ymd_res(2011, 10, 17)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 7)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 14)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2012, 1, 9)?,
                "Día de los Reyes Magos (observado)",
            ),
            (NaiveDate::from_ymd_res(2012, 3, 19)?, "Día de San José"),
            (NaiveDate::from_ymd_res(2012, 4, 5)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2012, 5, 21)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 6, 11)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2012, 6, 18)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 7, 2)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2012, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2012, 8, 20)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 15)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 5)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 12)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2013, 1, 7)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 3, 25)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 28)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2013, 5, 13)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 6, 3)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2013, 6, 10)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 7, 1)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2013, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2013, 8, 19)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 14)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 11, 4)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 11, 11)?,
                "Independencia de Cartagena",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2014, 1, 6)?,
                "Día de los Reyes Magos",
            ),
            (
                NaiveDate::from_ymd_res(2014, 3, 24)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 17)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2014, 6, 2)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 6, 23)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2014, 6, 30)?,
                "Sagrado Corazón (observado); San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2014, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2014, 8, 18)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 13)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 11, 3)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 11, 17)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2015, 1, 12)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 3, 23)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 2)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2015, 5, 18)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 6, 8)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2015, 6, 15)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2015, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2015, 8, 17)?,
                "La Asunción (observado)",
            ),
            (NaiveDate::from_ymd_res(2015, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2015, 11, 2)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 11, 16)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2016, 1, 11)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 3, 21)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 24)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2016, 5, 9)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 30)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2016, 6, 6)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 4)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2016, 8, 7)?, "Batalla de Boyacá"),
            (NaiveDate::from_ymd_res(2016, 8, 15)?, "La Asunción"),
            (
                NaiveDate::from_ymd_res(2016, 10, 17)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 11, 7)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 11, 14)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2017, 1, 9)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 3, 20)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 13)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2017, 5, 29)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 19)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2017, 6, 26)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 7, 3)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2017, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2017, 8, 21)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 16)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 11, 6)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 11, 13)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2018, 1, 8)?,
                "Día de los Reyes Magos (observado)",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 19)?, "Día de San José"),
            (NaiveDate::from_ymd_res(2018, 3, 29)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2018, 5, 14)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 4)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2018, 6, 11)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 7, 2)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2018, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2018, 8, 20)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 15)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 5)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 12)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2019, 1, 7)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 3, 25)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 18)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2019, 6, 3)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 6, 24)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2019, 7, 1)?,
                "Sagrado Corazón (observado); San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2019, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2019, 8, 19)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 14)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 4)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 11)?,
                "Independencia de Cartagena",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2020, 1, 6)?,
                "Día de los Reyes Magos",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 23)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 9)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2020, 5, 25)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 6, 15)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2020, 6, 22)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2020, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2020, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2020, 8, 17)?,
                "La Asunción (observado)",
            ),
            (NaiveDate::from_ymd_res(2020, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2020, 11, 2)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 11, 16)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2021, 1, 11)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 3, 22)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 1)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2021, 5, 17)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 6, 7)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2021, 6, 14)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 5)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2021, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2021, 8, 16)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 18)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2021, 11, 15)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2022, 1, 10)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 21)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 14)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2022, 5, 30)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 6, 20)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2022, 6, 27)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 4)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2022, 8, 7)?, "Batalla de Boyacá"),
            (NaiveDate::from_ymd_res(2022, 8, 15)?, "La Asunción"),
            (
                NaiveDate::from_ymd_res(2022, 10, 17)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 11, 7)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 11, 14)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2023, 1, 9)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 3, 20)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 6)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2023, 5, 22)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 12)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2023, 6, 19)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 7, 3)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2023, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2023, 8, 21)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 16)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 11, 6)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 11, 13)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2024, 1, 8)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 3, 25)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 28)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2024, 5, 13)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 3)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2024, 6, 10)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 7, 1)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2024, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2024, 8, 19)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 14)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 11, 4)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 11, 11)?,
                "Independencia de Cartagena",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2025, 1, 6)?,
                "Día de los Reyes Magos",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 24)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 17)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2025, 6, 2)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 23)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2025, 6, 30)?,
                "Sagrado Corazón (observado); San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2025, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2025, 8, 18)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 10, 13)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 11, 3)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 11, 17)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2026, 1, 12)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 23)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 2)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2026, 5, 18)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 6, 8)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2026, 6, 15)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2026, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2026, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2026, 8, 17)?,
                "La Asunción (observado)",
            ),
            (NaiveDate::from_ymd_res(2026, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2026, 11, 2)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 11, 16)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2027, 1, 11)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 22)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 25)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2027, 5, 10)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 31)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2027, 6, 7)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 7, 5)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2027, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2027, 8, 16)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 10, 18)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2027, 11, 15)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2028, 1, 10)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 3, 20)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 13)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2028, 5, 29)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 6, 19)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2028, 6, 26)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 7, 3)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2028, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2028, 8, 21)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 16)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 11, 6)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 11, 13)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2029, 1, 8)?,
                "Día de los Reyes Magos (observado)",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 19)?, "Día de San José"),
            (NaiveDate::from_ymd_res(2029, 3, 29)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2029, 5, 14)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 6, 4)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2029, 6, 11)?,
                "Sagrado Corazón (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 2)?,
                "San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2029, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2029, 8, 20)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 10, 15)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 11, 5)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 11, 12)?,
                "Independencia de Cartagena (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2030, 1, 7)?,
                "Día de los Reyes Magos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 3, 25)?,
                "Día de San José (observado)",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 18)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2030, 6, 3)?,
                "Ascensión del señor (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 6, 24)?,
                "Corpus Christi (observado)",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2030, 7, 1)?,
                "Sagrado Corazón (observado); San Pedro y San Pablo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 20)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2030, 8, 7)?, "Batalla de Boyacá"),
            (
                NaiveDate::from_ymd_res(2030, 8, 19)?,
                "La Asunción (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 10, 14)?,
                "Día de la Raza (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 11, 4)?,
                "Día de Todos los Santos (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 11, 11)?,
                "Independencia de Cartagena",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::CO,
        "Colombia",
    );

    Ok(map)
}
