# Import bulk users to LDAP server

Import users to your LDAP server from a CSV file.<br>
This program automatically converts the country field to its alpha2 code. <br>
It also supports LDAP with TLS/SSL (LDAPS) 


## Build
```bash
git clone https://github.com/paolobettelini/ldap-bulk-csv-rs
cd ldap-bulk-csv-rs
cargo build --release
# The executable is in ./target/release/ 
./target/release/bulk-ldap-rs --help
```

## Usage
Bulk CSV importer<br>
<br>
**<u>Usage</u>: bulk_ldap** [OPTIONS]<br>
<br>
**<u>Options:</u>**<br>
&emsp;&emsp;**-a, --address**              Service address [default: ldap://192.168.1.25:389]<br>
&emsp;&emsp;**-u, --user**                 Binding user DN [default: cn=svc_ldap,cn=Users,dc=BlackSky,dc=local]<br>
&emsp;&emsp;**-p, --password**             Binding user password [default: Admin123]<br>
&emsp;&emsp;**-c, --csv**                  CSV file [default: bulk.csv]<br>
&emsp;&emsp;**-h, --help**                 Print help information<br>
&emsp;&emsp;**-V, --version**              Print version information<br>