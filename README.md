# Import bulk users to LDAP server

## Build
```bash
git clone https://github.com/paolobettelini/ldap-bulk-csv-rs
cd ldap-bulk-csv-rs
cargo build --release
./target/release/bulk-ldap-rs --help
```

## Usage
Bulk CSV importer<br>
<br>
**<u>Usage</u>: bulk_ldap** [OPTIONS]<br>
<br>
**<u>Options:</u>**<br>
&emsp;&emsp;**-a, --address** <ADDRESS>    Service address [default: ldap://192.168.1.25:389]<br>
&emsp;&emsp;**-u, --user** <USER>          Binding user DN [default: cn=svc_ldap,cn=Users,dc=BlackSky,dc=local]<br>
&emsp;&emsp;**-p, --password** <PASSWORD>  Binding user password [default: Admin123]<br>
&emsp;&emsp;**-c, --csv** <CSV>            CSV file [default: bulk.csv]<br>
&emsp;&emsp;**-h, --help**                 Print help information<br>
&emsp;&emsp;**-V, --version**              Print version information<br>