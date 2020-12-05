use std::collections::HashMap;
use std::fs::File;
use std::io::{Read};
use regex::Regex;

pub static REQUIRED_FIELDS_PART_ONE: [&str;7] = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
];

fn main() {
    test_validate_credential();
    test_count_valid();
    test_validate_passport();
    test_validate_passports_count();

    let data = read_data("src/input.txt");

    let round_1_valid = count_valid(&data, &REQUIRED_FIELDS_PART_ONE.to_vec());
    println!("Valid records in round 1: {}", round_1_valid);

    let round_2_valid = count_valid_v2(&data, &REQUIRED_FIELDS_PART_ONE.to_vec());
    println!("Valid records in round 2: {}", round_2_valid);
}


fn read_data(filename: &str) -> Vec<HashMap<String, String>> {
    let mut file = File::open(filename)
        .expect("Something went wrong reading the file");
    let mut records_block = String::new();
    file.read_to_string(&mut records_block).expect("Error reading data");

    let records = build_credentials(records_block);

    return records;

}

fn validate_credential(credential: &HashMap<String, String>, required_fields: &Vec<&str>) -> bool {
    let mut valid = true;
    for field in required_fields{
        if !credential.contains_key(&field.to_string()) {
            valid = false;
            break;
        }
    }
    return valid;
}

fn count_valid(credentials: &Vec<HashMap<String, String>>, required_fields: &Vec<&str>) -> i64 {
    let validate = |credential| validate_credential(credential, required_fields);
    let mut valid = 0;
    for credential in credentials {
        if validate(credential) {
            valid+=1;
        }
    }
    return valid;
}

fn count_valid_v2(credentials: &Vec<HashMap<String, String>>, required_fields: &Vec<&str>) -> i64 {
    let validate = |credential| validate_credential(credential, required_fields);
    let mut valid = 0;
    for credential in credentials {
        if validate(credential) {
            let passport = Passport::new(credential);
            if passport.validate_all() {
                valid+=1;
            }
        }
    }
    return valid;
}

struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

impl Passport {
    pub fn new(record: &HashMap<String, String>) -> Passport {
            Passport {
                byr: record.get("byr").unwrap().parse().unwrap(),
                iyr: record.get("iyr").unwrap().parse().unwrap(),
                eyr: record.get("eyr").unwrap().parse().unwrap(),
                hgt: record.get("hgt").unwrap().to_string(),
                hcl: record.get("hcl").unwrap().to_string(),
                ecl: record.get("ecl").unwrap().to_string(),
                pid: record.get("pid").unwrap().to_string(),
            }

    }

    pub fn validate_all(&self) -> bool {
        return self.validate_birth_year()
            && self.validate_expiration_year()
            && self.validate_eye_color()
            && self.validate_hair_color()
            && self.validate_height()
            && self.validate_issue_year()
            && self.validate_passport_id();
    }

    pub fn validate_birth_year(&self) -> bool {
        return self.byr >= 1920 && self.byr <= 2002
    }

    pub fn validate_issue_year(&self) -> bool {
        return self.iyr >= 2010 && self.iyr <= 2020
    }

    pub fn validate_expiration_year(&self) -> bool {
        return self.eyr >= 2020 && self.eyr <= 2030
    }

    pub fn validate_height(&self) -> bool {
        if self.hgt.ends_with("cm") {
            let height: i16 = self.hgt.strip_suffix("cm").unwrap().parse().unwrap();
            return height >= 150 && height <= 193;
        }
        else if self.hgt.ends_with("in") {
            let height: i16 = self.hgt.strip_suffix("in").unwrap().parse().unwrap();
            return height >= 59 && height <= 76;
        }
        return false;
    }

    pub fn validate_hair_color(&self) -> bool {
        let re = Regex::new(r"^#[[0-9][a-f]]{6}$").unwrap();
        return re.is_match(&self.hcl);
    }

    pub fn validate_eye_color(&self) -> bool {
        let valid_colors: HashMap<String, bool> = [
            ("amb".to_string(), true),
            ("blu".to_string(), true),
            ("brn".to_string(), true),
            ("gry".to_string(), true),
            ("grn".to_string(), true),
            ("hzl".to_string(), true),
            ("oth".to_string(), true),
        ].iter().cloned().collect();

        return valid_colors.contains_key(&self.ecl);
    }

    pub fn validate_passport_id(&self) -> bool {
        let re = Regex::new(r"^[0-9]{9}$").unwrap();
        return re.is_match(&self.pid);
    }
}

fn build_credential(line: String) -> HashMap<String, String> {
    let mut credential = HashMap::new();

    for field in line.split_whitespace() {
        let key_value = field.split(':').collect::<Vec<&str>>();
        credential.entry(key_value[0].to_string()).or_insert(key_value[1].to_string());
    }

    return credential;
}

fn build_credentials(block: String) -> Vec<HashMap<String, String>> {
    let mut credentials = Vec::new();

    for record in block.split("\n\n"){
        credentials.push(build_credential(record.to_string()));
    }

    return credentials;
}

fn test_validate_credential() {
    let credential = build_credential("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm".to_string());
    assert_eq!(validate_credential(&credential, &REQUIRED_FIELDS_PART_ONE.to_vec()), true);

    let credential_2 = build_credential("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    hcl:#cfa07d byr:1929".to_string());
    assert_eq!(validate_credential(&credential_2, &REQUIRED_FIELDS_PART_ONE.to_vec()), false);

    let credential_3 = build_credential("hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm".to_string());
    assert_eq!(validate_credential(&credential_3, &REQUIRED_FIELDS_PART_ONE.to_vec()), true);

    let credential_4 = build_credential("hcl:#cfa07d eyr:2025 pid:166559648
    iyr:2011 ecl:brn hgt:59in".to_string());
    assert_eq!(validate_credential(&credential_4, &REQUIRED_FIELDS_PART_ONE.to_vec()), false);

}

fn test_count_valid() {
    let credential_block = 
"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in".to_string();

    let credentials = build_credentials(credential_block);
    assert_eq!(count_valid(&credentials, &REQUIRED_FIELDS_PART_ONE.to_vec()), 2)
}

fn test_validate_passport() {
    let credential = build_credential("eyr:1972 cid:100
    hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926".to_string());
    let passport = Passport::new(&credential);
    assert_eq!(passport.validate_all(), false);

    let credential_2 = build_credential("pid:087499704 
    hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
    hcl:#623a2f".to_string());
    let passport_2 = Passport::new(&credential_2);
    assert_eq!(passport_2.validate_all(), true);

    let credential_2 = build_credential("pid:087499704
    hgt:74in ecl:grn iyr:2012 eyr:20300 byr:1980
    hcl:#623a2f".to_string());
    let passport_2 = Passport::new(&credential_2);
    assert_eq!(passport_2.validate_all(), false);

    let credential_2 = build_credential("pid:087499704
    hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
    hcl:#623a2x".to_string());
    let passport_2 = Passport::new(&credential_2);
    assert_eq!(passport_2.validate_all(), false);

    let credential_2 = build_credential("pid:0874997049 
    hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
    hcl:#623a2f".to_string());
    let passport_2 = Passport::new(&credential_2);
    assert_eq!(passport_2.validate_all(), false);

    let credential_2 = build_credential("pid:087499704 
    hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
    hcl:#623a2f4".to_string());
    let passport_2 = Passport::new(&credential_2);
    assert_eq!(passport_2.validate_all(), false);
}

fn test_validate_passports_count() {
    let credential_block = "
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".to_string();

    let credentials = build_credentials(credential_block);
    assert_eq!(count_valid_v2(&credentials, &REQUIRED_FIELDS_PART_ONE.to_vec()), 4);

    let credential_block_no_valid = "
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007".to_string();

    let credentials_no_valid = build_credentials(credential_block_no_valid);
    assert_eq!(count_valid_v2(&credentials_no_valid, &REQUIRED_FIELDS_PART_ONE.to_vec()), 0);
}
