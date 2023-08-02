<script setup>
import Statistics from "../components/Statistics.vue";
import Footer from "../components/Footer.vue";
import SessionList from "../components/SessionList.vue";
import SessionModal from "../components/SessionModal.vue";
import {ref, watch} from "vue";
import VehicleSelect from "../components/VehicleSelect.vue";

const open = ref(false);
const period = ref("7d");
const statisticsComponent = ref(null);
const sessionListComponent = ref(null);


const updateData = () => {
  statisticsComponent.value.refresh();
  sessionListComponent.value.refresh();
}

</script>

<template>
  <div class="pt-5">
    <div class="flex flex-wrap items-center gap-6 px-4 sm:flex-nowrap sm:px-6 lg:px-8">
      <h1 class="text-base font-semibold leading-7 text-gray-900">Statty</h1>
      <div class="order-last flex w-full gap-x-8 text-sm font-semibold leading-6 sm:order-none sm:w-auto sm:border-l sm:border-gray-200 sm:pl-6 sm:leading-7">
        <a href="#" @click="period = '7d'" :class="period === '7d' ? 'text-indigo-600' : 'text-gray-700'">Last 7 days</a>
        <a href="#" @click="period = '30d'" :class="period === '30d' ? 'text-indigo-600' : 'text-gray-700'">Last 30 days</a>
        <a href="#" @click="period = 'all'" :class="period === 'all' || !period || period === '' ? 'text-indigo-600' : 'text-gray-700'">All-time</a>
      </div>
      <!--      <a href="#" class="ml-auto flex items-center gap-x-1 rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">-->
      <!--        <PlusSmallIcon class="-ml-1.5 h-5 w-5" aria-hidden="true" />-->
      <!--        New invoice-->
      <!--      </a>-->
      <VehicleSelect class="ml-auto flex items-center gap-x-1 rounded-md px-3 py-2"></VehicleSelect>
    </div>
  </div>
  <div class="container mx-auto">
    <SessionModal @toggleModal="open = false" @submitSuccess="updateData" :open="open"></SessionModal>
    <Statistics ref="statisticsComponent" class="py-5" :period="period"></Statistics>
    <SessionList ref="sessionListComponent" @updateList="updateData" @toggleModal="open = !open" class="py-5" :period="period"></SessionList>
  </div>
  <footer>
    <Footer></Footer>
  </footer>
</template>

<style scoped>

</style>