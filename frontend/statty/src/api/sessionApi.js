import {Client} from "./http.js";

export const SessionApi = {
    getSessions,
    addSession,
    deleteSession
}

function getSessions(vehicleId, period) {
    let query = "";
    if (period) {
        query = `?period=${period}`
    }
    return Client.get(`/ev_stats/vehicles/${vehicleId}/charging_sessions${query}`);
}

function addSession(vehicleId, session) {
    return Client.post(`/ev_stats/vehicles/${vehicleId}/charging_sessions`, session);
}

//delete session
function deleteSession(vehicleId, sessionId) {
    return Client.delete(`/ev_stats/vehicles/${vehicleId}/charging_sessions/${sessionId}`);
}