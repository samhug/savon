#[macro_use]
extern crate log;

use std::env;

mod soap {
    include!(concat!(env!("OUT_DIR"), "/country_info_service.rs"));
}

mod soap1 {
    include!(concat!(env!("OUT_DIR"), "/example.rs"));
}

#[tokio::main]
async fn main() -> Result<(), savon::Error> {
    pretty_env_logger::init();
    trace!("OUT_DIR:{}", env!("OUT_DIR"));
    let base_url = "http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso";
    info!("Hello, world!");

    let client = soap::CountryInfoService::new(base_url.to_string());

    let res = client
        .list_of_continents_by_name(soap::messages::ListOfContinentsByNameSoapRequest(
            soap::types::ListOfContinentsByName {},
        ))
        .await?;

    info!("res: {:?}", res);

    Ok(())
}
