<script setup>
import Statistics from "../components/Statistics.vue";
import Footer from "../components/Footer.vue";
import SessionList from "../components/SessionList.vue";
import Header from "../components/Header.vue";
import SessionModal from "../components/SessionModal.vue";
import {ref} from "vue";

const open = ref(false);

</script>

<template>
  <Header class="pt-5"></Header>
  <div class="container mx-auto">
    <h1>{{ open }}</h1>
    <SessionModal @toggleModal="open = false" :open="open"></SessionModal>
    <Statistics class="py-5"></Statistics>
    <SessionList @toggleModal="open = !open" class="py-5"></SessionList>
    <Footer class="absolute inset-x-0 bottom-0"></Footer>
  </div>
</template>

<style scoped>

</style>