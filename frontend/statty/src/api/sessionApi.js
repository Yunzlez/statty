import {Client} from "./http.js";

export const SessionApi = {
    getSessions,
    addSession
}

function getSessions(vehicleId) {
    return Client.get(`/ev_stats/vehicles/${vehicleId}/charging_sessions`);
}

function addSession(vehicleId, session) {
    return Client.post(`/ev_stats/vehicles/${vehicleId}/charging_sessions`, session);
}