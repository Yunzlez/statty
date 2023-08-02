import {Client} from "./http.js";

export const StatsApi = {
    getStats
}

function getStats(vehicleId, period) {
    let query = "";
    if (period) {
        query = `?period=${period}`
    }
    return Client.get(`/ev_stats/vehicles/${vehicleId}/stats${query}`)
}