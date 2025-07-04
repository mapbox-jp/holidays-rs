#!/usr/bin/env python3

import holidays
from dataclasses import dataclass
import csv
import datetime
import hashlib


current_year = datetime.date.today().year

# Make sure to also update ./build.rs constants.
# This range should be more LESS conservative to ensure holidays.csv contains
# all data that might be needed.
years = list(range(0, current_year + 11))


@dataclass
class Country:
    code: str
    name: str


# Read countries from CSV
def read_countries(countries_csv):
    countries = []
    with open(countries_csv, newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        for row in reader:
            code = row["ISO 3166-1 A2"].strip().upper()
            name = row["Name"].strip()
            if code:
                countries.append(Country(code, name))
    return countries


def gen_holiday_csv(countries, output_csv, years):
    all_holidays = []

    for country in countries:
        try:
            HolidayClass = getattr(holidays, country.code)
        except AttributeError:
            print(f"No holiday class found for country code: {country.code}")
            continue

        holiday_data = HolidayClass(years=years)
        for date, name in holiday_data.items():
            all_holidays.append((date, country.code, name))

    # Sort by date, then country_code, then holiday name
    all_holidays.sort(key=lambda x: (x[0], x[1], x[2]))

    with open(output_csv, mode="w", newline="", encoding="utf-8") as f:
        writer = csv.writer(f)
        writer.writerow(["country_code", "date", "holiday_name"])  # Header

        for date, country_code, name in all_holidays:
            writer.writerow([country_code, date.isoformat(), name])

    print(f"[OK] CSV generated: {output_csv}")


def write_csv_hash(csv_path):
    hasher = hashlib.sha256()
    with open(csv_path, "rb") as f:
        hasher.update(f.read())
    hash_hex = hasher.hexdigest()

    hash_path = csv_path + ".hash"
    with open(hash_path, "w") as f:
        f.write(hash_hex + "\n")

    print(f"[OK] Wrote hash to: {hash_path}")


def update_cargo_toml(cargo_toml_path, countries):
    country_codes = list(sorted(map(lambda it: it.code, countries)))

    with open(cargo_toml_path, "r", encoding="utf-8") as f:
        lines = f.readlines()

    start_index = None
    for i, line in enumerate(lines):
        if line.strip().startswith("all-countries"):
            start_index = i
            break

    if start_index is None:
        raise ValueError("Could not find `all-countries` in Cargo.toml")

    # Truncate from all-countries line onward
    lines = lines[:start_index]

    def sorted_repr(seq):
        return "[\n  " + ",\n  ".join(f'"{x}"' for x in seq) + "\n]\n"

    # Prepare new lines
    lines.append("\n")
    lines.append(f"all-countries = {sorted_repr(country_codes)}\n")
    for code in country_codes:
        lines.append(f"{code} = []\n")

    # Write back
    with open(cargo_toml_path, "w", encoding="utf-8") as f:
        f.writelines(lines)

    print(f"[OK] Updated features in {cargo_toml_path}")


countries_csv = "countries.csv"
output_csv = "holidays.csv"

if __name__ == "__main__":
    countries = read_countries(countries_csv)
    gen_holiday_csv(countries, output_csv, years)
    write_csv_hash(output_csv)
    update_cargo_toml("Cargo.toml", countries)
