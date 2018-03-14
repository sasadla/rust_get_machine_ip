extern crate machine_ip;//import to help get ip address
use std::process::Command;//rust built in library to launch a command in windows cmd or linux bash
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};//rust default library to help identify ip address type 
use std::net::ToSocketAddrs;//rust default library to deal with socket addresses
use std::io;//rust default input output library
use std::str;//rust library to deal with strings
/*
* Main function that runs the app and launches the rocket app server
*/
fn main() {
		println!("Ip-Address is: {}", get_host_ip().unwrap().to_string());
}

/*
* Custom function created to generically get the ip address from the given host the app runs on
*/
fn get_host_ip() -> Option<IpAddr> {
    let output = Command::new("hostname")//launch a command on cmd or bash to get the hosts name i.e the machine name e.g rustpc
        .output()
        .expect("failed to execute `hostname`");//generate error if the command fails to execute
	let std_out = String::from_utf8(output.stdout).unwrap();//get the returned output from the outcome of the above command as the host name string
	let std_res =std_out.trim();//trim the output string to get rid of any semicolons etc to just leave the host name
	let name_to_ip_res :Vec<IpAddr>=name_to_ip(std_res).unwrap();//use another predefined function to get the list of all the socket addresses in the running machine or vm
	let host_ip_option = name_to_ip_res.last();//it was found that the host ip adapter socket always shows at the end of the list or is the sole one in the list
	//either way extracting the last value of the list gives us the desired ip address
	let host_ip = host_ip_option.unwrap().to_string();//extract the host ip address from the option wrapping
	let ips: Vec<&str> = host_ip.trim().split(" ").collect::<Vec<&str>>();//generate a vector to test for the ip address type
	let first = ips.first();//generate an option type to work with the match statement
	match first{
        Some(first) =>  {
            if !first.is_empty(){//confirm if no null values are present in the option
                if let Ok(addr) = first.parse::<Ipv4Addr>() {//if ip type ip v4 return as ip v4 option
                    return Some(IpAddr::V4(addr))
                }
                else if let Ok(addr) = first.parse::<Ipv6Addr>() {//if ip type ip v6 return as ip v6 option
                    return Some(IpAddr::V6(addr))
                }
                else{
                    None//if the ip type not matched to above values return null value
                }
            }else{
                None//if option empty return null value
            }
        }
        None => None//if null value to be returned that return option with null value
    }
}

/*
* Custom function to get the host name as a string and generate a vector list of all the socket addresses present on it
*/
fn name_to_ip(host: &str) -> io::Result<Vec<IpAddr>> {
    (host, 0).to_socket_addrs().map(|iter| iter.map(|socket_address| socket_address.ip()).collect())//run a map iterator to generate 
	//a vector of the list of all the socket address on the given host name
}