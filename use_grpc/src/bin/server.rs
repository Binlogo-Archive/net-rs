use std::thread;
use use_grpc::foobar::*;
use use_grpc::foobar_grpc::*;

fn main() {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(9001);
    server.add_service(FooBarServiceServer::new_service_def(FooBarServer));
    let _server = server.build().expect("Start server failed");
    loop {
        thread::park()
    }
}

struct FooBarServer;
impl FooBarService for FooBarServer {
    fn record_cab_location(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<CabLocationRequest>,
        resp: ::grpc::ServerResponseUnarySink<CabLocationResponse>,
    ) -> ::grpc::Result<()> {
        println!(
            "Record cab {} at {}, {}",
            req.message.get_name(),
            req.message.get_location().latitude,
            req.message.get_location().longitude,
        );
        let mut res = CabLocationResponse::new();
        res.set_accepted(true);
        resp.finish(res)
    }

    fn get_cabs(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<GetCabRequest>,
        resp: ::grpc::ServerResponseUnarySink<GetCabResponse>,
    ) -> ::grpc::Result<()> {
        println!(
            "Get cabs around: {}, {}",
            req.message.get_location().latitude,
            req.message.get_location().longitude
        );
        let mut res = GetCabResponse::new();

        let mut location = Location::new();
        location.latitude = 345.043;
        location.longitude = 34.084;

        let mut cab = Cab::new();
        cab.set_name("Suco".to_owned());
        cab.set_location(location.clone());

        res.set_cabs(protobuf::RepeatedField::from_vec(vec![cab]));

        resp.finish(res)
    }
}
