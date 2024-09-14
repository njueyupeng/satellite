export * from "./pkg/satellite.js";
import {get_constants} from './pkg/satellite.js'

export const constants = JSON.parse(get_constants());