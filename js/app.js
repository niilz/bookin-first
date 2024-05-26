import init, {
  BookingServiceWasm,
  FetchApiClient,
  CookieWasm,
  LoginCreds,
} from "../pkg/fitness_api.js";

async function run() {
  console.log("> run");
  await init();
  console.log("ran init");

  let httpClient = new FetchApiClient();
  console.log({ httpClient });
  let cookieJar = new CookieWasm();
  console.log({ cookieJar });
  let bookingService = new BookingServiceWasm(httpClient, cookieJar);
  console.log({ bookingService });

  let res = await bookingService.login("USER-NAME", "PASSWORD");

  console.log({ res });
}

run();
