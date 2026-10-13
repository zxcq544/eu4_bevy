import json


class Country:
    def __init__(self, name, short_name_tag_3_letters, flag_index):
        self.name = name
        self.short_name_tag_3_letters = short_name_tag_3_letters
        self.flag_index = flag_index

    def __repr__(self):
        return f"{self.name} ({self.short_name_tag_3_letters}) {self.flag_index}"

    def __str__(self):
        return f"{self.name} ({self.short_name_tag_3_letters}) {self.flag_index}"


def build_countries_from_json_files(flags_json_file, countries_json_file):
    with open(flags_json_file, "r", encoding="utf-8") as f:
        flags_json = json.load(f)

    with open(countries_json_file, "r", encoding="utf-8") as f:
        countries_json = json.load(f)

    country_tags = flags_json["flags"]

    countries_json = {k: v for k, v in sorted(countries_json.items())}

    countries = []

    for country_tag, _ in country_tags.items():
        if country_tag in countries_json:
            country = Country(
                countries_json[country_tag],
                country_tag,
                flags_json["flags"][country_tag]["index"],
            )
        else:
            country = Country(
                country_tag, country_tag, flags_json["flags"][country_tag]["index"]
            )
        countries.append(country)

    return countries


if __name__ == "__main__":
    flags_json_file = "../assets/data/flags.json"
    countries_json_file = "../assets/data/countries.json"

    countries = build_countries_from_json_files(flags_json_file, countries_json_file)

    with open("../assets/data/result_countries.json", "w", encoding="utf-8") as f:
        json.dump(
            countries, f, default=lambda o: o.__dict__, indent=4, ensure_ascii=False
        )

    print(countries[0:5])
