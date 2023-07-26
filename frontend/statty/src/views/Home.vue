<script setup>
import Statistics from "../components/Statistics.vue";
import Footer from "../components/Footer.vue";
import SessionList from "../components/SessionList.vue";
import Header from "../components/Header.vue";

</script>

<template>
  <Header class="pt-5"></Header>
  <div class="container mx-auto">
    <Statistics class="py-5"></Statistics>
    <SessionList class="py-5"></SessionList>
    <Footer class="absolute inset-x-0 bottom-0"></Footer>
  </div>
</template>

<style scoped>

</style>