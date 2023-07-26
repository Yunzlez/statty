<template>
  <div>
    <h3 class="text-base font-semibold leading-6 text-gray-900 dark:text-white">Last 30 days</h3>
    <dl class="mt-5 grid grid-cols-1 divide-y divide-gray-200 overflow-hidden rounded-lg bg-white shadow md:grid-cols-4 md:divide-x md:divide-y-0">
      <div v-for="item in stats" :key="item.name" class="px-4 py-5 sm:p-6">
        <dt class="text-base font-normal text-gray-900">{{ item.name }}</dt>
        <dd class="mt-1 flex items-baseline justify-between md:block lg:flex">
          <div class="flex items-baseline text-2xl font-semibold text-indigo-600">
            {{ item.stat }}<span v-if="item.unit"> {{ item.unit }}</span>
            <span v-if="item.previousStat" class="ml-2 text-sm font-medium text-gray-500">from {{ item.previousStat }}<span v-if="item.unit"> {{ item.unit }}</span></span>
          </div>

          <div v-if="item.change" :class="[item.changeType === 'increase' ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800', 'inline-flex items-baseline rounded-full px-2.5 py-0.5 text-sm font-medium md:mt-2 lg:mt-0']">
            <ArrowUpIcon v-if="item.changeType === 'increase'" class="-ml-1 mr-0.5 h-5 w-5 flex-shrink-0 self-center text-green-500" aria-hidden="true" />
            <ArrowDownIcon v-else class="-ml-1 mr-0.5 h-5 w-5 flex-shrink-0 self-center text-red-500" aria-hidden="true" />
            <span class="sr-only"> {{ item.changeType === 'increase' ? 'Increased' : 'Decreased' }} by </span>
            {{ item.change }}
          </div>
        </dd>
      </div>
    </dl>
  </div>
</template>

<script setup>
import { ArrowDownIcon, ArrowUpIcon } from '@heroicons/vue/20/solid'
import {onMounted, ref} from 'vue';
import {StatsApi} from "../api/statsApi.js";

// const stats = [
//   { id: 1, name: 'Average consumption since last charge', stat: '3.14 kWh', previousStat: '3 kWh', change: '5%', changeType: 'increase' },
//   { id: 2, name: 'Average consumption overall', stat: '3.14 kWh', previousStat: '3 kWh', change: '5%', changeType: 'increase' },
//   { id: 3, name: 'Registered sessions', stat: '96', previousStat: '93', change: '3', changeType: 'increase' },
//   { id: 4, name: 'Total km driven', stat: '1500 km', previousStat: '1250', change: '250', changeType: 'increase' },
// ]

let stats = ref(0);
onMounted(() => {
  StatsApi.getStats(1).then(res => {
    let apiStats = [];
    apiStats.push({ id: 1, name: 'Average consumption since last charge', stat: Number(res.data.avg_consumption_last_charge.toFixed(2)) , previousStat: '', change: '', changeType: 'increase', unit: 'kWh' });
    apiStats.push({ id: 2, name: 'Average consumption overall', stat: Number(res.data.avg_consumption.toFixed(2)) , previousStat: '', change: '', changeType: 'increase', unit: 'kWh' });
    apiStats.push({ id: 3, name: 'Registered sessions', stat: res.data.num_sessions , previousStat: '', change: '', changeType: 'increase' });
    apiStats.push({ id: 4, name: 'Total km driven', stat: res.data.total_distance , previousStat: '', change: '', changeType: 'increase', unit: 'km' });

    stats.value = apiStats;
  });
});
</script>