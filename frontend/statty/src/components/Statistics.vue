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
            <ArrowUpIcon v-if="item.changeType === 'increase' && !item.inversePositive" class="-ml-1 mr-0.5 h-5 w-5 flex-shrink-0 self-center text-green-500" aria-hidden="true" />
            <ArrowDownIcon v-else :class="item.inversePositive ? 'text-red-500' : 'text-green-500' " class="-ml-1 mr-0.5 h-5 w-5 flex-shrink-0 self-center" aria-hidden="true" />
            <span class="sr-only"> {{ item.changeType === 'increase' ? 'Increased' : 'Decreased' }} by </span>
            {{ item.change }} {{item.unit}}
          </div>
        </dd>
      </div>
    </dl>
  </div>
</template>

<script setup>
import { ArrowDownIcon, ArrowUpIcon } from '@heroicons/vue/20/solid'
import {onMounted, ref, watch} from 'vue';
import {StatsApi} from "../api/statsApi.js";

let stats = ref(0);
const props = defineProps({
  period: String
});

onMounted(() => {
  refresh();
});

watch(() => props.period, async () => {
  console.log("refreshing stats");
  await refresh();
})

const refresh = async () => {
  StatsApi.getStats(1, props.period || '').then(res => {
    let apiStats = [];
    apiStats.push({ id: 1, name: 'Total charged', stat: Number(res.data.total_energy.toFixed(2)) , previousStat: '', change: '', changeType: 'increase', unit: 'kWh' });
    apiStats.push({ id: 2, name: 'Average consumption overall', stat: Number(res.data.avg_consumption.toFixed(2)) , previousStat: '', change: '', changeType: 'increase', inversePositive: true, unit: 'kWh' });
    apiStats.push({ id: 3, name: 'Registered sessions', stat: res.data.num_sessions , previousStat: '', change: '', changeType: 'increase' });
    apiStats.push({ id: 4, name: 'Total km driven', stat: res.data.total_distance , previousStat: '', change: '', changeType: 'increase', unit: 'km' });

    stats.value = apiStats;
  });

  StatsApi.getStats(1, props.period || '', true).then(res => {
    let old_total_energy = res.data.total_energy.toFixed(2);
    let total_energy = stats.value[0].stat;
    stats.value[0].previousStat = old_total_energy;
    stats.value[0].change = (total_energy - old_total_energy)?.toFixed(2);
    stats.value[0].changeType = total_energy > old_total_energy ? 'increase' : 'decrease';

    let old_avg_consumption = Number(res.data.avg_consumption.toFixed(2));
    let avg_consumption = stats.value[1].stat;
    stats.value[1].previousStat = old_avg_consumption;
    stats.value[1].change = (avg_consumption - old_avg_consumption).toFixed(2);
    stats.value[1].changeType = avg_consumption < old_avg_consumption ? 'increase' : 'decrease';

    let old_num_sessions = Number(res.data.num_sessions);
    let num_sessions = stats.value[2].stat;
    stats.value[2].previousStat = old_num_sessions;
    stats.value[2].change = (num_sessions - old_num_sessions);
    stats.value[2].changeType = num_sessions > old_num_sessions ? 'increase' : 'decrease';

    let old_total_distance = Number(res.data.total_distance);
    let total_distance = stats.value[3].stat;
    stats.value[3].previousStat = old_total_distance;
    stats.value[3].change = (total_distance - old_total_distance);
    stats.value[3].changeType = total_distance > old_total_distance ? 'increase' : 'decrease';
  })
}

defineExpose({refresh});

</script>