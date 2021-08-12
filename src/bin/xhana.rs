extern crate diesel;
extern crate pf4shconf;

use self::pf4shconf::*;
// use std::io;
// use clap::{load_yaml, App, AppSettings, Arg};
use clap::{load_yaml, App};

fn main() {
    //    use pf4shconf::schema::*;

    let connection = establish_connection();

    let yaml = load_yaml!("../xhana.yaml");

    let matches = App::from(yaml).get_matches();

    // The most common way to handle subcommands is via a combined approach using
    // `ArgMatches::subcommand` which returns a tuple of both the name and matches
    match matches.subcommand() {
        Some(("add", add_matches)) => {
            // Now we have a reference to push's matches
            
            match add_matches.subcommand() {
                Some(("parametertemplate", parameter_matches)) => {
                    // Now we have a reference to remote's matches
                    let version = parameter_matches.value_of("version").unwrap();
                    let info = parameter_matches.value_of("info").unwrap();
                    let parameter = parameter_matches.value_of("name").unwrap();
                    let typ = parameter_matches.value_of("type").unwrap();
                    let mandatory = parameter_matches.value_of("mandatory").unwrap();
                    let mandatory = mandatory.trim_end();
                    let scope = parameter_matches.value_of("scope").unwrap();
                    let iotype = parameter_matches.value_of("iotype").unwrap();
                    let arctype = parameter_matches.value_of("arctype").unwrap();
                    println!("Version: {}", version);
                    println!("Info to the technical parameter: {}", info);
                    println!("Technical Parametername: {}", parameter);
                    println!("Scope: {}", scope);
                    println!("Arctype: {}", arctype);
                    println!("IOType: {}", iotype);
                    println!("Type of Parametervalue: {}", typ);
                    println!("Mandatory: {}", mandatory);
                    add_xhanaparameter(
                        &connection,
                        &version,
                        &info,
                        &parameter,
                        &scope,
                        &arctype,
                        &iotype,
                        &typ,
                        &mandatory,
                    ); 
                    println!(
                        "\nSaved configversion {}, info {}, parameter {}, arctype {}, typ {}, mandatory {}",
                        version, info, parameter, arctype, typ, mandatory, 
                    );
                    //query_hanaparameter(&connection, &configversion);
                }
                
                Some(("architecture", parameter_matches)) => {
                    // Now we have a reference to remote's matches
                    let arctype = parameter_matches.value_of("arctype").unwrap();
                    println!("Arctype: {}", arctype);
                    add_xhanaarc(&connection, &arctype);
                }
                Some(("solution", parameter_matches)) => {
                    // Now we have a reference to remote's matches
                    let solutionversion = parameter_matches.value_of("description").unwrap();
                    let sid = parameter_matches.value_of("sid").unwrap();
                    let arctype = parameter_matches.value_of("arctype").unwrap();
                    let sidname =  match parameter_matches.occurrences_of("sidname") {
                        0 => "EMPTY",
                        _ => parameter_matches.value_of("sidname").unwrap()
                      };
                    let tag =  match parameter_matches.occurrences_of("tag") {
                        0 => "EMPTY",
                        _ => parameter_matches.value_of("tag").unwrap()
                      };

                    println!("Solutionversion: {}", solutionversion);
                    println!("SID: {}", sid);
                    println!("sidname: {}", sidname);
                    println!("arctype: {}", arctype);
                    println!("tag: {}", tag);    
                    
                    add_xhanasid(&connection, &sid, &sidname);
                    add_xhanasolution(&connection, &solutionversion);
                    add_xhana_solution_sid(&connection, &solutionversion, &sid, &arctype, &tag);
                }
                Some(("datacenter", parameter_matches)) => {
                    // Now we have a reference to remote's matches
                    let id_str = parameter_matches.value_of("id").unwrap();
                    let name = parameter_matches.value_of("dcname").unwrap();
                    let id: i32 = id_str.parse().expect("Error: ID is not a integer");
                    println!("ID: {}", id);
                    println!("Name: {}", name);
                    add_xhanadc(&connection, &id, &name);
                }
                Some(("version", parameter_matches)) => {
                    // Now we have a reference to remote's matches
                    let sid = parameter_matches.value_of("sid").unwrap();
                    let configversion = parameter_matches.value_of("configversion").unwrap();
                    let tag = parameter_matches.value_of("tag").unwrap();
                    println!("SID: {}", sid);
                    println!("Configversion: {}", configversion);
                    println!("Tag: {}", tag);
                    /*add_xhanaversion(&connection, &sid, &configversion, &tag);*/
                }
                Some(("model", parameter_matches)) => {
                    // Now we have a reference to remote's matches
                    
                    let file = parameter_matches.value_of("file").unwrap();
                    println!("File: {}", file);
                    /*add_xhanamodel(&connection, &file);*/
                    // add_xhanaversion(&connection, &sid, &configversion, &tag);
                }
                None => println!("No subcommand was used"),
                _ => unreachable!(),
            }
        }
        None => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    //println!("Value: ");
    //let mut value = String::new();
    //io::stdin().read_line(&mut value).unwrap();
    //let value = &value[..(value.len() - 1)];

    //let xhg = create_xhanageneral(&connection, &sid, &version, &parameter, &value);
}
