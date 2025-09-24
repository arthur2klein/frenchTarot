async fn http_handler(req: HttpRequest) -> HttpResponse {
    let dto = CliOrHttpDto::from(req);
    logging::Logger.handle(dto, |d| play_card(d))
}
