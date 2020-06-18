use std::env;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Provide a name.");
        std::process::exit(1);
    }

    let query = format!("{}", args[1]);
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let res = resolver.lookup_ip(query.as_str());
    for ip in res.iter() {
        println!("Default: {:?}", ip);
    }

    let resolver = Resolver::from_system_conf().unwrap();
    let res = resolver.lookup_ip(query.as_str());
    for ip in res.iter() {
        println!("System: {:?}", ip);
    }
}
