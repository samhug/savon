use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    savon::gen::gen_write("./CountryInfoService.wsdl", &out_dir).unwrap();
    savon::gen::gen_write("./example.wsdl", &out_dir).unwrap();
    savon::gen::gen_write("../assets/WHWebService.wsdl", &out_dir).unwrap();
}
