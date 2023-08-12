<template>
  <div class="px-4 sm:px-6 lg:px-8">
    <div class="sm:flex sm:items-center">
      <div class="sm:flex-auto">
        <h1 class="text-base font-semibold leading-6 text-gray-900">Charging sessions</h1>
        <p class="mt-2 text-sm text-gray-700">Charging session for {{ vehicle.name }} (id {{ vehicle.id }})</p>
      </div>
      <div class="mt-4 sm:ml-16 sm:mt-0 sm:flex-none">
        <button type="button" @click="$emit('toggleModal')" class="block rounded-md bg-indigo-600 px-3 py-2 text-center text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Add session</button>
      </div>
    </div>
    <div class="mt-8 flow-root">
      <div class="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
        <div class="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
          <div class="overflow-hidden shadow ring-1 ring-black ring-opacity-5 sm:rounded-lg">
            <table class="min-w-full divide-y divide-gray-300">
              <thead class="bg-gray-50">
              <tr>
                <th scope="col" class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6">Date</th>
                <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">Energy</th>
                <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">Odometer</th>
                <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">End SoC</th>
                <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">Distance</th>
                <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">Avg Consumption</th>
                <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">Actions</th>
<!--                <th scope="col" class="relative py-3.5 pl-3 pr-4 sm:pr-6">-->
<!--                  <span class="sr-only">Edit</span>-->
<!--                </th>-->
              </tr>
              </thead>
              <tbody class="divide-y divide-gray-200 bg-white">
              <tr v-for="session, i in sessions" :key="session.id">
                <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-6">{{ session.date }}</td>
                <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{{ session.energy }} kWh</td>
                <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{{ session.odometer }} km</td>
                <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{{ session.end_soc }} %</td>
                <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500"><span v-if="i !== (sessions.length -1)">{{ session.odometer - sessions[i+1].odometer }} km</span><span v-else> N/A </span></td>
<!--                TODO add clim calculation-->
                <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500"><span v-if="i !== (sessions.length -1)">{{ ((session.energy / (session.odometer - sessions[i+1].odometer)) * 100).toFixed(2) }} kWh/100 km</span><span v-else> N/A </span></td>
                <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                  <button @click="deleteSession(session.id)" type="button" class="inline-flex items-center px-2.5 py-1.5 border border-transparent text-xs font-medium rounded shadow-sm text-white bg-red-600 hover:bg-red-700 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-red-500">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0" />
                    </svg>
                  </button>
                </td>
<!--                <td class="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">-->
<!--                  <a href="#" class="text-indigo-600 hover:text-indigo-900"-->
<!--                  >Edit<span class="sr-only">, {{ session.id }}</span></a-->
<!--                  >-->
<!--                </td>-->
              </tr>
              </tbody>
            </table>
            <nav class="flex items-center justify-between border-t border-gray-200 bg-white px-4 py-3 sm:px-6" aria-label="Pagination">
              <div class="hidden sm:block">
                <p class="text-sm text-gray-700">
                  Showing
                  <span class="font-medium">{{ firstElementIndex }}</span>
                  to
                  <span class="font-medium">{{ lastElementIndex }}</span>
                  of
                  <span class="font-medium">{{ pageData?.total_items }}</span>
                  results
                </p>
              </div>
              <div class="flex flex-1 justify-between sm:justify-end">
                <a href="#" v-if="!firstPage" class="relative inline-flex items-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus-visible:outline-offset-0">Previous</a>
                <a href="#" v-if="!lastPage" class="relative ml-3 inline-flex items-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus-visible:outline-offset-0">Next</a>
              </div>
            </nav>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import {computed, onMounted, ref, watch} from "vue";
import {SessionApi} from "../api/sessionApi.js";

//todo update on insert

onMounted(async () => {
  await refresh();
});

const props = defineProps({
  period: String
});

watch(() => props.period, async () => {
  console.log("refreshing stats");
  await refresh();
})

let vehicle = ref();
vehicle.value = {
  id: 1,
  name: "Volvo C40"
};
let sessions = ref();
let pageData = ref({});
let pageSize = 10;

let firstElementIndex = computed(() => {
  return pageData.value.page * pageSize + 1;
});

let lastElementIndex = computed(() => {
  //if it's the last page
  if (lastPage) {
    return pageData.value.total_items;
  }

  return (pageData.value.page + 1) * pageSize;
});

//computed properties for first and last page
let firstPage = computed(() => {
  return pageData.value.page === 0;
});

let lastPage = computed(() => {
  return pageData.value.page + 1 >= pageData.value.total_pages;
});

const emit = defineEmits(['updateList'])

const deleteSession = async (sessionId) => {
  SessionApi.deleteSession(1, sessionId)
      .then(() => {
        emit('updateList');
        refresh();
      });
  //todo error handling
}

const refresh = async () => {
  let data = (await SessionApi.getSessions(1, props.period)).data;
  sessions.value = data.items;
  pageData.value = data.meta;
}

defineExpose({refresh})

</script>