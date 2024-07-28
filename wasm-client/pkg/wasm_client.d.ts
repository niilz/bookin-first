/* tslint:disable */
/* eslint-disable */
/**
* @param {string} user_name
* @param {string} password
* @param {string} mode
* @returns {Promise<any>}
*/
export function login(user_name: string, password: string, mode: string): Promise<any>;
/**
* @param {string} session_id
* @param {string} user_id
* @returns {Promise<any>}
*/
export function courses(session_id: string, user_id: string): Promise<any>;
/**
* @param {string} session_id
* @param {string} course_id
* @returns {Promise<any>}
*/
export function slots(session_id: string, course_id: string): Promise<any>;
/**
* @param {BookingRequest} booking_request
* @param {string} session_id
* @param {string} user_id
* @param {boolean} cancel
* @returns {Promise<any>}
*/
export function book_course(booking_request: BookingRequest, session_id: string, user_id: string, cancel: boolean): Promise<any>;
/**
* @param {string} user_id
* @param {number} slot_id
* @param {number} course_id
* @param {string} course_name
* @returns {BookingRequest}
*/
export function create_booking_request(user_id: string, slot_id: number, course_id: number, course_name: string): BookingRequest;
/**
*/
export class BookingRequest {
  free(): void;
/**
*/
  course_id: number;
/**
*/
  slot_id: number;
/**
*/
  user_id: number;
}
/**
*/
export class CourseWithSlot {
  free(): void;
/**
*/
  booked: boolean;
/**
*/
  end_date_time: bigint;
/**
* Course:Slot-Id in the form "12345678:12345678"
*/
  id: string;
/**
*/
  max_capacity: number;
/**
* Name of the course
*/
  name: string;
/**
*/
  start_date_time: bigint;
/**
*/
  total_booked: number;
}
/**
*/
export class CourseWrapper {
  free(): void;
/**
*/
  course: CourseWithSlot;
}
/**
*/
export class NetpulseLoginResponse {
  free(): void;
/**
*/
  chain_name: string;
/**
*/
  chain_uuid: string;
/**
*/
  club_id: string;
/**
*/
  club_name: string;
/**
*/
  email_varified: boolean;
/**
*/
  first_name: string;
/**
*/
  last_name: string;
/**
*/
  membership_type: string;
/**
*/
  profile_completed: boolean;
/**
*/
  timezone: string;
/**
*/
  timezone_offset: number;
/**
*/
  user_id: string;
/**
*/
  verified: boolean;
}
/**
*/
export class SimpleCourse {
  free(): void;
/**
* Whether the course is bookable or not
*/
  bookable: boolean;
/**
* like "freestyle Kleingruppentraining"
*/
  category: string;
/**
*/
  description: string;
/**
* Course-Duration in minutes
*/
  duration: number;
/**
* Internal-Id of the course
*/
  id: number;
/**
* Image-URL starts with "https://"
*/
  image_url: string;
/**
* Name of the course
*/
  title: string;
/**
* like "Studio"
*/
  typ: string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly login: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly courses: (a: number, b: number, c: number, d: number) => number;
  readonly slots: (a: number, b: number, c: number, d: number) => number;
  readonly book_course: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly __wbg_netpulseloginresponse_free: (a: number) => void;
  readonly __wbg_get_netpulseloginresponse_user_id: (a: number, b: number) => void;
  readonly __wbg_set_netpulseloginresponse_user_id: (a: number, b: number, c: number) => void;
  readonly __wbg_get_netpulseloginresponse_first_name: (a: number, b: number) => void;
  readonly __wbg_set_netpulseloginresponse_first_name: (a: number, b: number, c: number) => void;
  readonly __wbg_get_netpulseloginresponse_last_name: (a: number, b: number) => void;
  readonly __wbg_set_netpulseloginresponse_last_name: (a: number, b: number, c: number) => void;
  readonly __wbg_get_netpulseloginresponse_verified: (a: number) => number;
  readonly __wbg_set_netpulseloginresponse_verified: (a: number, b: number) => void;
  readonly __wbg_get_netpulseloginresponse_email_varified: (a: number) => number;
  readonly __wbg_set_netpulseloginresponse_email_varified: (a: number, b: number) => void;
  readonly __wbg_get_netpulseloginresponse_club_id: (a: number, b: number) => void;
  readonly __wbg_set_netpulseloginresponse_club_id: (a: number, b: number, c: number) => void;
  readonly __wbg_get_netpulseloginresponse_club_name: (a: number, b: number) => void;
  readonly __wbg_set_netpulseloginresponse_club_name: (a: number, b: number, c: number) => void;
  readonly __wbg_get_netpulseloginresponse_chain_uuid: (a: number, b: number) => void;
  readonly __wbg_set_netpulseloginresponse_chain_uuid: (a: number, b: number, c: number) => void;
  readonly __wbg_get_netpulseloginresponse_chain_name: (a: number, b: number) => void;
  readonly __wbg_set_netpulseloginresponse_chain_name: (a: number, b: number, c: number) => void;
  readonly __wbg_get_netpulseloginresponse_timezone: (a: number, b: number) => void;
  readonly __wbg_set_netpulseloginresponse_timezone: (a: number, b: number, c: number) => void;
  readonly __wbg_get_netpulseloginresponse_timezone_offset: (a: number) => number;
  readonly __wbg_set_netpulseloginresponse_timezone_offset: (a: number, b: number) => void;
  readonly __wbg_get_netpulseloginresponse_profile_completed: (a: number) => number;
  readonly __wbg_set_netpulseloginresponse_profile_completed: (a: number, b: number) => void;
  readonly __wbg_get_netpulseloginresponse_membership_type: (a: number, b: number) => void;
  readonly __wbg_set_netpulseloginresponse_membership_type: (a: number, b: number, c: number) => void;
  readonly __wbg_bookingrequest_free: (a: number) => void;
  readonly __wbg_get_bookingrequest_user_id: (a: number) => number;
  readonly __wbg_set_bookingrequest_user_id: (a: number, b: number) => void;
  readonly __wbg_get_bookingrequest_slot_id: (a: number) => number;
  readonly __wbg_set_bookingrequest_slot_id: (a: number, b: number) => void;
  readonly __wbg_get_bookingrequest_course_id: (a: number) => number;
  readonly __wbg_set_bookingrequest_course_id: (a: number, b: number) => void;
  readonly create_booking_request: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly __wbg_simplecourse_free: (a: number) => void;
  readonly __wbg_get_simplecourse_id: (a: number) => number;
  readonly __wbg_set_simplecourse_id: (a: number, b: number) => void;
  readonly __wbg_get_simplecourse_title: (a: number, b: number) => void;
  readonly __wbg_set_simplecourse_title: (a: number, b: number, c: number) => void;
  readonly __wbg_get_simplecourse_typ: (a: number, b: number) => void;
  readonly __wbg_set_simplecourse_typ: (a: number, b: number, c: number) => void;
  readonly __wbg_get_simplecourse_duration: (a: number) => number;
  readonly __wbg_set_simplecourse_duration: (a: number, b: number) => void;
  readonly __wbg_get_simplecourse_category: (a: number, b: number) => void;
  readonly __wbg_set_simplecourse_category: (a: number, b: number, c: number) => void;
  readonly __wbg_get_simplecourse_description: (a: number, b: number) => void;
  readonly __wbg_set_simplecourse_description: (a: number, b: number, c: number) => void;
  readonly __wbg_get_simplecourse_image_url: (a: number, b: number) => void;
  readonly __wbg_set_simplecourse_image_url: (a: number, b: number, c: number) => void;
  readonly __wbg_get_simplecourse_bookable: (a: number) => number;
  readonly __wbg_set_simplecourse_bookable: (a: number, b: number) => void;
  readonly __wbg_get_coursewrapper_course: (a: number) => number;
  readonly __wbg_set_coursewrapper_course: (a: number, b: number) => void;
  readonly __wbg_coursewithslot_free: (a: number) => void;
  readonly __wbg_get_coursewithslot_id: (a: number, b: number) => void;
  readonly __wbg_set_coursewithslot_id: (a: number, b: number, c: number) => void;
  readonly __wbg_get_coursewithslot_name: (a: number, b: number) => void;
  readonly __wbg_set_coursewithslot_name: (a: number, b: number, c: number) => void;
  readonly __wbg_get_coursewithslot_start_date_time: (a: number) => number;
  readonly __wbg_set_coursewithslot_start_date_time: (a: number, b: number) => void;
  readonly __wbg_get_coursewithslot_end_date_time: (a: number) => number;
  readonly __wbg_set_coursewithslot_end_date_time: (a: number, b: number) => void;
  readonly __wbg_get_coursewithslot_max_capacity: (a: number) => number;
  readonly __wbg_set_coursewithslot_max_capacity: (a: number, b: number) => void;
  readonly __wbg_get_coursewithslot_booked: (a: number) => number;
  readonly __wbg_set_coursewithslot_booked: (a: number, b: number) => void;
  readonly __wbg_get_coursewithslot_total_booked: (a: number) => number;
  readonly __wbg_set_coursewithslot_total_booked: (a: number, b: number) => void;
  readonly __wbg_coursewrapper_free: (a: number) => void;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hc1c4f16c3a7d8f72: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h4289acb35cbe71a1: (a: number, b: number, c: number, d: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
