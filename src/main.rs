use serde::{Deserialize, Serialize};
use serde_json::json;
use sysinfo;


#[derive(Debug,Serialize, Deserialize)]
struct PackageDescriptor{
    name:String,
    path:String,
    description:Option<String>
}


#[derive(Debug,Serialize, Deserialize)]
struct PSConfig{
    packages:Vec<PackageDescriptor>
}
trait JsonDecoder{
    fn decode_json(json:String)->Self;
}
impl JsonDecoder for PackageDescriptor{
    fn decode_json(json:String)->Self{
        return serde_json::from_str(&json).unwrap();
    }
}
impl JsonDecoder for PSConfig{
    fn decode_json(json:String)->Self{
        return serde_json::from_str(&json).unwrap();
    }
}
#[derive(Debug,Serialize, Deserialize)]
enum PackageType{
    Media,
    Application,
    Library,
    Documnet
}
#[derive(Debug,Serialize, Deserialize)]
enum CpuArchs{
    Amd64,
    X86,
    Arm64,
    All,
}
#[derive(Debug,Serialize, Deserialize)]
enum OS{
    Windows,
    Linux,
    BSD,
    MacOS,
    Unix
}
#[derive(Debug,Serialize, Deserialize)]
struct Reqs{
    ram:Option<u64>, // in Bytes
    cpu_cores:Option<u32>,
    free_disk_space:Option<u64>, // in Bytes 
    os:Option<OS>
}
impl JsonDecoder for Reqs{
    fn decode_json(json:String)->Self {
        return serde_json::from_str(&json).unwrap();
    }
}
#[derive(Debug,Serialize, Deserialize)]
struct PckgInfo{
    ptype:PackageType,
    content:String,
    cpu_arch:CpuArchs,
    requirements:Option<Reqs>
}
impl JsonDecoder for PckgInfo{
    fn decode_json(json:String)->Self {
        return serde_json::from_str(&json).unwrap();
    }
    
}
#[derive(Debug,Serialize, Deserialize)]
struct UserSystemInfo{
    cores:u32,
    ram:u64,
    disk_free_space:u64,
    arch:CpuArchs,
    os:OS,
}
fn get_os()->OS{
    let bsd_family = vec!["freebsd","netbsd","openbsd"];
    const COS:&str = std::env::consts::OS;
    if COS == "windows"{
        return OS::Windows;
    }else if COS == "linux"{
        return OS::Linux;
    }else if bsd_family.contains(&COS){
        return OS::BSD;
    }else if COS == "macos"{
        return OS::MacOS;
    }
    return OS::Unix;
}
fn get_cpu_arch()->CpuArchs{
    if std::env::consts::ARCH == "x86"{
        return CpuArchs::X86;
    }else if std::env::consts::ARCH == "x86_64"{
        return CpuArchs::Amd64;
    }
    return CpuArchs::Arm64;
}
impl UserSystemInfo{
    
    pub fn get()->Self{
        let disks = sysinfo::Disks::new_with_refreshed_list();
        let mut sys = sysinfo::System::new();
        sys.refresh_cpu();
        sys.refresh_memory();

        return Self {
            cores: sys.cpus().len() as u32,
            ram: sys.available_memory(),
            disk_free_space: disks.get(0).unwrap().available_space(),
            arch: get_cpu_arch() ,
            os: get_os()
        }
    }
}
fn main() {

}
