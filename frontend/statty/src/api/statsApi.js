import {Client} from "./http.js";

export const StatsApi = {
    getStats
}

function getStats(vehicleId, period, prev) {
    let query = "";
    let hasParams = false;

    const queryParams = new URLSearchParams({});
    if (period) {
        queryParams.append("period", period);
        hasParams = true;
    }
    if (prev) {
        queryParams.append("prev", prev);
        hasParams = true;
    }

    query = `?${queryParams.toString()}`;

    return Client.get(`/ev_stats/vehicles/${vehicleId}/stats${query}`)
}