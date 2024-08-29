use serde::{Deserialize, Serialize};
use serde_json::json;
use sysinfo;
use reqwest;
use std::io::Write;
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
impl PSConfig{
    pub fn get_package_path_from_name(&mut self,name:String)->String{
        for i in &self.packages{
            if i.name == name{
                return i.path.clone();
            }
        }
        let s = String::new();
        return s;
    }
    pub fn search_package(&mut self,keyword:String)->Vec<String>{
        let mut out :Vec<String>= Vec::new();
        for i in &self.packages{
            if i.name.contains(&keyword){
                out.push(i.path.clone());
            }
            else if i.path.contains(&keyword){
                out.push(i.path.clone());
                continue;
            }
            match &i.description{
                Some(e)=>{
                    if e.contains(&keyword){
                        out.push(i.path.clone())
                    }

                },
                None=>{}
            }
        }
        return out;
    }
}
#[derive(Debug,Serialize, Deserialize)]
enum PackageType{
    Media,
    Application,
    Library,
    Documnet
}
#[derive(Debug,Serialize, Deserialize,PartialEq)]
enum CpuArchs{
    Amd64,
    X86,
    Arm64,
    All,
}
#[derive(Debug,Serialize, Deserialize,PartialEq)]
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
struct PkgInfo{
    name:String,
    ptype:PackageType,
    content:String,
    cpu_arch:CpuArchs,
    requirements:Option<Reqs>
}
impl JsonDecoder for PkgInfo{
    fn decode_json(json:String)->Self {
        return serde_json::from_str(&json).unwrap();
    }
    
}
impl PkgInfo{
    pub fn check_hardware(&mut self)->bool{
        let user = UserSystemInfo::get();
        if self.cpu_arch == CpuArchs::All || self.cpu_arch == user.arch{
            match &self.requirements{
                Some(e)=>{
                    match &e.ram{Some(r_am)=>{if !(*r_am <= user.ram){
                        println!("you need more ram space to get this package");return false;}},None=>{}}
                    match &e.cpu_cores{Some(c_cpu_cores)=>{if !(*c_cpu_cores <= user.cores){
                        println!("you need more cpu cores to get this package");return false;}},None=>{}}
                    match &e.free_disk_space{Some(free_disk_space)=>{if !(*free_disk_space <= user.disk_free_space){
                        println!("you need more free disk space to get this package");return false;}},None=>{}}
                    match &e.os{Some(os)=>{if !(user.os == *os){
                        println!("this package is not for your OS");return false}},None=>{}}
                    return true;
                },
                None=>{},
            }
            return true
        }
        println!("this package is not for your cpu");
        return false;
    }
    pub async fn download(&mut self){
        println!("hardware checking ....");
        if !self.check_hardware() {
            println!("hardware check is failed !");
            return;
        }
        download_from_net(self.content.clone(),self.name.clone()).await;

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
async fn download_from_net(url:String,path:String) {
    println!("get from:   {}",url);
    let out = reqwest::get(url.clone()).await.unwrap().bytes().await.unwrap();
    let mut n_path = path.clone();
    let splitted = url.split(".").collect::<Vec<&str>>();
    let extension = splitted[splitted.len() - 1];
    n_path.push('.');
    n_path.push_str(extension);
    let mut f = std::fs::File::create(n_path).unwrap();
    f.write_all(&out);
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





async fn init_zpm()->PSConfig{
    let pm_config_url = "https://raw.githubusercontent.com/0xangoone/apm/main/zpkgs.io/Packages/config.json".to_string();
    let pm_config_json =reqwest::get(pm_config_url).await.unwrap().text().await.unwrap();
    let mut pkgs_config=  PSConfig::decode_json(pm_config_json);
    return pkgs_config;
}
async fn download_p(p_name:String){
    println!("init the package manger ...");
    let mut pkgs_config=init_zpm().await;

    println!("searching about package ....");
    let p_path = pkgs_config.get_package_path_from_name(p_name.clone());
    if p_path.is_empty(){
        println!("{} is not found !",p_name);
        return;
    }
    println!("found {}",p_name);


    let mut p_info_url = p_path.clone();
    p_info_url.push_str("/info.json");
    let p_info_json = reqwest::get( p_info_url.clone() ).await.unwrap().text().await.unwrap();
    println!("get the package configuration ....");
    let mut p_info = PkgInfo::decode_json(p_info_json);
    println!("downloading to disk......");
    p_info.download().await;
    println!("Package was downloaded successfully ");
}
async fn search_p(keyword:String){
    println!("init the package manger ...");
    let mut pkgs_config=init_zpm().await;

    println!("searching about package ....");
    let mut result = pkgs_config.search_package(keyword);
    if result.is_empty(){
        println!("package was not found!");
    }
    println!("search resaults: ");
    for mut i in &mut result{
        i.push_str("/info.json");
        let p_info_json = reqwest::get( i.clone() ).await.unwrap().text().await.unwrap();
        let mut p_info = PkgInfo::decode_json(p_info_json);
        println!("{:#?}",p_info);

    }
}
#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    if args.len() <= 2{
        return;
    }
    let option = args.nth(1).unwrap();
    if option == "install"{
        download_p(args.nth(0).unwrap()).await;
    }else if option == "search"{
        search_p(args.nth(0).unwrap()).await;
    }
    
}
