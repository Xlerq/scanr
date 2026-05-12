use std::io::{self, Write};

use crate::models::{Config, OutputFormat, ScanSummary};

pub fn print_summary(summary: &ScanSummary, config: &Config) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    write_summary(&mut handle, summary, config).expect("failed to write scan summary");
}

fn write_summary<W: Write>(
    writer: &mut W,
    summary: &ScanSummary,
    config: &Config,
) -> io::Result<()> {
    match config.format {
        OutputFormat::Table => write_table_summary(writer, summary, config),
        OutputFormat::Json => write_json_summary(writer, summary, config),
        OutputFormat::Csv => write_csv_summary(writer, summary, config),
    }
}

fn write_table_summary<W: Write>(
    writer: &mut W,
    summary: &ScanSummary,
    config: &Config,
) -> io::Result<()> {
    if summary.open_ports.is_empty() {
        writeln!(writer, "\n\nNo open ports found")?;
    } else {
        writeln!(writer, "\n\n PORT  SERVICE")?;
        writeln!(writer, "===============")?;

        for open in summary.open_ports.iter() {
            let port_name: Option<&str> = get_common_port_name(*open);

            match port_name {
                Some(name) => writeln!(writer, "{:>5}  {}", open, name)?,
                None => writeln!(writer, " {:>5}", open)?,
            }
        }
    }

    writeln!(writer, "===============\n")?;
    writeln!(writer, "Ip: {}", config.ip)?;

    let scanned_ports_number: usize = config.ports.len();

    if scanned_ports_number != 1 {
        writeln!(writer, "Scanned: {} ports", config.ports.len())?;
    } else {
        writeln!(writer, "Scanned: 1 port")?;
    }

    writeln!(writer, "Elapsed: {} s", summary.elapsed.as_secs_f32())?;

    Ok(())
}

fn write_csv_summary<W: Write>(
    writer: &mut W,
    summary: &ScanSummary,
    config: &Config,
) -> io::Result<()> {
    writeln!(writer, "ip,port,service")?;

    for open in summary.open_ports.iter() {
        let port_name: Option<&str> = get_common_port_name(*open);

        match port_name {
            Some(name) => writeln!(writer, "{},{},{}", config.ip, open, name)?,
            None => writeln!(writer, "{},{}", config.ip, open)?,
        }
    }

    Ok(())
}

fn write_json_summary<W: Write>(
    writer: &mut W,
    summary: &ScanSummary,
    config: &Config,
) -> io::Result<()> {
    let mut common_port_names: Vec<&str> = Vec::with_capacity(summary.open_ports.len());

    for open in summary.open_ports.iter() {
        let port_name: Option<&str> = get_common_port_name(*open);

        match port_name {
            Some(name) => common_port_names.push(name),
            None => common_port_names.push(""),
        }
    }

    writeln!(writer, "{{")?;
    writeln!(writer, "  \"ip\": \"{}\",", config.ip)?;
    writeln!(writer, "  \"open_ports\": {:?}", summary.open_ports)?;
    writeln!(writer, "  \"service_name\": {:?}", common_port_names)?;
    writeln!(writer, "}}")?;

    Ok(())
}

fn get_common_port_name(port: u16) -> Option<&'static str> {
    let index = COMMON_PORTS
        .iter()
        .position(|common_port| common_port.0 == port);

    match index {
        Some(i) => Some(COMMON_PORTS[i].1),
        None => None,
    }
}

const COMMON_PORTS: &[(u16, &str)] = &[
    (20, "ftp-data"),
    (21, "ftp"),
    (22, "ssh"),
    (23, "telnet"),
    (25, "smtp"),
    (37, "time"),
    (43, "whois"),
    (49, "tacacs"),
    (53, "dns"),
    (67, "dhcp-server"),
    (68, "dhcp-client"),
    (69, "tftp"),
    (70, "gopher"),
    (79, "finger"),
    (80, "http"),
    (88, "kerberos"),
    (102, "iso-tsap"),
    (110, "pop3"),
    (111, "rpcbind"),
    (113, "ident"),
    (119, "nntp"),
    (123, "ntp"),
    (135, "msrpc"),
    (137, "netbios-ns"),
    (138, "netbios-dgm"),
    (139, "netbios-ssn"),
    (143, "imap"),
    (161, "snmp"),
    (162, "snmptrap"),
    (179, "bgp"),
    (194, "irc"),
    (389, "ldap"),
    (443, "https"),
    (445, "smb"),
    (465, "smtps"),
    (500, "isakmp"),
    (514, "syslog"),
    (515, "printer"),
    (520, "rip"),
    (587, "submission"),
    (631, "ipp"),
    (636, "ldaps"),
    (873, "rsync"),
    (902, "vmware-auth"),
    (989, "ftps-data"),
    (990, "ftps"),
    (993, "imaps"),
    (995, "pop3s"),
    (1080, "socks"),
    (1194, "openvpn"),
    (1433, "mssql"),
    (1434, "mssql-monitor"),
    (1521, "oracle"),
    (1723, "pptp"),
    (1883, "mqtt"),
    (2049, "nfs"),
    (2082, "cpanel"),
    (2083, "cpanel-ssl"),
    (2086, "whm"),
    (2087, "whm-ssl"),
    (2181, "zookeeper"),
    (2375, "docker"),
    (2376, "docker-tls"),
    (2483, "oracle"),
    (2484, "oracle-ssl"),
    (3000, "dev-http"),
    (3306, "mysql"),
    (3389, "rdp"),
    (3690, "svn"),
    (4369, "epmd"),
    (5000, "upnp"),
    (5060, "sip"),
    (5061, "sip-tls"),
    (5353, "mdns"),
    (5355, "llmnr"),
    (5432, "postgresql"),
    (5672, "amqp"),
    (5900, "vnc"),
    (5984, "couchdb"),
    (5985, "winrm-http"),
    (5986, "winrm-https"),
    (6379, "redis"),
    (6443, "kubernetes-api"),
    (6667, "irc"),
    (7001, "weblogic"),
    (8000, "http-alt"),
    (8008, "http-alt"),
    (8080, "http-proxy"),
    (8081, "http-alt"),
    (8086, "influxdb"),
    (8443, "https-alt"),
    (8888, "http-alt"),
    (9000, "php-fpm"),
    (9090, "prometheus"),
    (9092, "kafka"),
    (9093, "alertmanager"),
    (9100, "node-exporter"),
    (9200, "elasticsearch"),
    (9300, "elasticsearch-transport"),
    (9418, "git"),
    (10250, "kubelet"),
    (10255, "kubelet-readonly"),
    (11211, "memcached"),
    (15672, "rabbitmq-management"),
    (27017, "mongodb"),
    (27018, "mongodb-shard"),
    (27019, "mongodb-config"),
];
