use std::{collections::HashSet, fs::File, str};
use clap::Parser;
use ldap3::{LdapConn, LdapConnSettings};
use csv::{ByteRecord, ReaderBuilder};

mod record;
mod countries;
mod args;

use record::Record;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Args::parse();

    // Connect and bind to LDAP server
    let mut settings = LdapConnSettings::new();
    if args.address.contains("ldaps") {
        settings = settings.set_starttls(true);
    }
    let mut ldap = LdapConn::with_settings(settings, &args.address)?;
    ldap.simple_bind(&args.user, &args.password)?
        .success()?;

    // Read CSV
    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(File::open(&args.csv)?);
    let mut raw_record = ByteRecord::new();
    let headers = reader.byte_headers()?.clone();

    // Process records
    while reader.read_byte_record(&mut raw_record)? {
        let record: Record = raw_record.deserialize(Some(&headers))?;

        // Create OU iteratively
        let created = create_ou_iter(&mut ldap, &record.oupath);
        if created.is_err() {
            println!("OU path is not valid. Skipping...");
            continue;
        }

        let cn = str::from_utf8(&record.username)?;
        let path = str::from_utf8(&record.oupath)?;
        let dn = format!("CN={},{}", cn, path);

        // Delete user
        let _ = ldap.delete(&dn);

        fn add_if_present<'a>(
            vec: &mut Vec<(&'a [u8], HashSet<&'a [u8]>)>,
            attr_name: &'a str,
            val: &'a [u8],
        ) {
            if !val.is_empty() {
                vec.push((attr_name.as_bytes(), HashSet::from([val])));
            }
        }

        let mut values = vec![];

        add_if_present(&mut values, "objectClass", "User".as_bytes());
        add_if_present(&mut values, "name", &record.username[..]);
        add_if_present(&mut values, "givenname", &record.first_name[..]);
        add_if_present(&mut values, "initials", &record.middle_initial[..]);
        add_if_present(&mut values, "sn", &record.last_name[..]);
        add_if_present(&mut values, "samaccountname", &record.username[..]);
        add_if_present(&mut values, "mail", &record.email[..]);
        add_if_present(&mut values, "streetAddress", &record.street_address[..]);
        add_if_present(&mut values, "l", &record.city[..]);
        add_if_present(&mut values, "st", &record.state[..]);
        add_if_present(&mut values, "department", &record.department[..]);
        add_if_present(&mut values, "userpassword", &record.password[..]);
        add_if_present(&mut values, "telephonenumber", &record.telephone[..]);
        add_if_present(&mut values, "title", &record.job_title[..]);
        add_if_present(&mut values, "company", &record.company[..]);

        // Set country field
        if let Ok(country) = str::from_utf8(&record.country) {
            if let Some(alpha2) = countries::get_alpha_2(country) {
                add_if_present(&mut values, "c", alpha2.as_bytes());
            }
        }

        // Add user
        let res = ldap.add(&dn, values);

        // Log result
        let response = if let Ok(ldap_result) = res {
            ldap_result.text
        } else {
            "Error".to_owned()
        };
        println!("Created user {dn},\nresponse: ({response})\n");
    }

    Ok(())
}

fn create_ou_iter(
    ldap_connection: &mut LdapConn,
    path: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let str_path = str::from_utf8(path)?;
    println!("Creating OU(s) {str_path}");

    let index = str_path.to_lowercase().find("dc").unwrap();
    let ous: Vec<&str> = str_path[..(index - 1)].split(',').collect();
    let dc: &str = &str_path[(index)..];
    let mut complete_path = "".to_owned();

    for (i, ou) in ous.iter().rev().enumerate() {
        complete_path = if i == 0 {
            ou.to_string()
        } else {
            format!("{},{}", ou, complete_path)
        };

        let ou = &ou[3..];
        let dn = format!("{},{}", complete_path, dc);

        let _ = ldap_connection.add(
            &dn,
            vec![
                ("ou", HashSet::from([ou])),
                ("name", HashSet::from([ou])),
                ("objectClass", HashSet::from(["organizationalUnit"])),
            ],
        );
    }

    Ok(())
}
