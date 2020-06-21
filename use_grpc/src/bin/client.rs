use use_grpc::foobar::*;
use use_grpc::foobar_grpc::*;

use futures::executor;
use grpc::ClientStubExt;

fn main() -> std::io::Result<()> {
    let client = FooBarServiceClient::new_plain("127.0.0.1", 9001, Default::default()).unwrap();
    let mut req = CabLocationRequest::new();
    req.set_name("foo".to_owned());

    let mut location = Location::new();
    location.latitude = 345.043;
    location.longitude = 34.084;
    req.set_location(location);

    let resp = client
        .record_cab_location(Default::default(), req)
        .join_metadata_result();
    let resp = executor::block_on(resp);
    match resp {
        Ok((_, r, _)) => println!("Record cab res: {:?}", r),
        Err(e) => eprint!("Record cab err: {}", e),
    }

    let mut nearby_req = GetCabRequest::new();
    let mut location = Location::new();
    location.latitude = 40.730610;
    location.longitude = -73.935242;
    nearby_req.set_location(location);

    let nearby_resp = client
        .get_cabs(grpc::RequestOptions::new(), nearby_req)
        .join_metadata_result();
    let nearby_resp = executor::block_on(nearby_resp);
    match nearby_resp {
        Err(e) => panic!("{:?}", e),
        Ok((_, cabs, _)) => println!("Nearby: {:?}", cabs),
    }

    Ok(())
}
