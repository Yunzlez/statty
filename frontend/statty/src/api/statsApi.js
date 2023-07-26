import {Client} from "./http.js";

export const StatsApi = {
    getStats
}

function getStats(vehicleId) {
    return Client.get(`/ev_stats/vehicles/${vehicleId}/stats`)
}