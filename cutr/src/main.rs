fn main() {
    if let Err(e) = cutr::get_args().and_then(curt::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
