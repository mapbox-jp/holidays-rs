mod helper;

use crate::{prelude::*, Holiday, NaiveDateExt, Result, Year};
use helper::build_year;

use chrono::NaiveDate;
use std::collections::BTreeMap;
use std::collections::HashMap;

#[cfg(feature = "AO")]
pub mod ao;

#[cfg(feature = "AR")]
pub mod ar;

#[cfg(feature = "AM")]
pub mod am;

#[cfg(feature = "AW")]
pub mod aw;

#[cfg(feature = "AU")]
pub mod au;

#[cfg(feature = "AT")]
pub mod at;

#[cfg(feature = "AZ")]
pub mod az;

#[cfg(feature = "BD")]
pub mod bd;

#[cfg(feature = "BY")]
pub mod by;

#[cfg(feature = "BE")]
pub mod be;

#[cfg(feature = "BO")]
pub mod bo;

#[cfg(feature = "BA")]
pub mod ba;

#[cfg(feature = "BW")]
pub mod bw;

#[cfg(feature = "BR")]
pub mod br;

#[cfg(feature = "BG")]
pub mod bg;

#[cfg(feature = "BI")]
pub mod bi;

#[cfg(feature = "CA")]
pub mod ca;

#[cfg(feature = "CL")]
pub mod cl;

#[cfg(feature = "CN")]
pub mod cn;

#[cfg(feature = "CO")]
pub mod co;

#[cfg(feature = "HR")]
pub mod hr;

#[cfg(feature = "CU")]
pub mod cu;

#[cfg(feature = "CW")]
pub mod cw;

#[cfg(feature = "CY")]
pub mod cy;

#[cfg(feature = "CZ")]
pub mod cz;

#[cfg(feature = "DK")]
pub mod dk;

#[cfg(feature = "DJ")]
pub mod dj;

#[cfg(feature = "DO")]
pub mod r#do;

#[cfg(feature = "EG")]
pub mod eg;

#[cfg(feature = "EE")]
pub mod ee;

#[cfg(feature = "ET")]
pub mod et;

#[cfg(feature = "FI")]
pub mod fi;

#[cfg(feature = "FR")]
pub mod fr;

#[cfg(feature = "GE")]
pub mod ge;

#[cfg(feature = "DE")]
pub mod de;

#[cfg(feature = "GR")]
pub mod gr;

#[cfg(feature = "HN")]
pub mod hn;

#[cfg(feature = "HK")]
pub mod hk;

#[cfg(feature = "HU")]
pub mod hu;

#[cfg(feature = "IS")]
pub mod is;

#[cfg(feature = "IN")]
pub mod r#in;

#[cfg(feature = "ID")]
pub mod id;

#[cfg(feature = "IE")]
pub mod ie;

#[cfg(feature = "IM")]
pub mod im;

#[cfg(feature = "IL")]
pub mod il;

#[cfg(feature = "IT")]
pub mod it;

#[cfg(feature = "JM")]
pub mod jm;

#[cfg(feature = "JP")]
pub mod jp;

#[cfg(feature = "KZ")]
pub mod kz;

#[cfg(feature = "KE")]
pub mod ke;

#[cfg(feature = "LV")]
pub mod lv;

#[cfg(feature = "LS")]
pub mod ls;

#[cfg(feature = "LI")]
pub mod li;

#[cfg(feature = "LT")]
pub mod lt;

#[cfg(feature = "LU")]
pub mod lu;

#[cfg(feature = "MG")]
pub mod mg;

#[cfg(feature = "MY")]
pub mod my;

#[cfg(feature = "MW")]
pub mod mw;

#[cfg(feature = "MT")]
pub mod mt;

#[cfg(feature = "MX")]
pub mod mx;

#[cfg(feature = "MD")]
pub mod md;

#[cfg(feature = "MA")]
pub mod ma;

#[cfg(feature = "MZ")]
pub mod mz;

#[cfg(feature = "NL")]
pub mod nl;

#[cfg(feature = "NA")]
pub mod na;

#[cfg(feature = "NZ")]
pub mod nz;

#[cfg(feature = "NI")]
pub mod ni;

#[cfg(feature = "NG")]
pub mod ng;

#[cfg(feature = "MK")]
pub mod mk;

#[cfg(feature = "NO")]
pub mod no;

#[cfg(feature = "PK")]
pub mod pk;

#[cfg(feature = "PY")]
pub mod py;

#[cfg(feature = "PE")]
pub mod pe;

#[cfg(feature = "PL")]
pub mod pl;

#[cfg(feature = "PT")]
pub mod pt;

#[cfg(feature = "RO")]
pub mod ro;

#[cfg(feature = "RU")]
pub mod ru;

#[cfg(feature = "SA")]
pub mod sa;

#[cfg(feature = "RS")]
pub mod rs;

#[cfg(feature = "SG")]
pub mod sg;

#[cfg(feature = "SK")]
pub mod sk;

#[cfg(feature = "SI")]
pub mod si;

#[cfg(feature = "ZA")]
pub mod za;

#[cfg(feature = "KR")]
pub mod kr;

#[cfg(feature = "ES")]
pub mod es;

#[cfg(feature = "SZ")]
pub mod sz;

#[cfg(feature = "SE")]
pub mod se;

#[cfg(feature = "CH")]
pub mod ch;

#[cfg(feature = "TW")]
pub mod tw;

#[cfg(feature = "TR")]
pub mod tr;

#[cfg(feature = "TN")]
pub mod tn;

#[cfg(feature = "UA")]
pub mod ua;

#[cfg(feature = "AE")]
pub mod ae;

#[cfg(feature = "GB")]
pub mod gb;

#[cfg(feature = "US")]
pub mod us;

#[cfg(feature = "UY")]
pub mod uy;

#[cfg(feature = "UZ")]
pub mod uz;

#[cfg(feature = "VE")]
pub mod ve;

#[cfg(feature = "VN")]
pub mod vn;

#[cfg(feature = "ZM")]
pub mod zm;

#[cfg(feature = "ZW")]
pub mod zw;
