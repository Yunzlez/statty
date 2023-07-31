<script setup>
import Statistics from "../components/Statistics.vue";
import Footer from "../components/Footer.vue";
import SessionList from "../components/SessionList.vue";
import Header from "../components/Header.vue";
import SessionModal from "../components/SessionModal.vue";
import {ref} from "vue";

const open = ref(false);
const statisticsComponent = ref(null);
const sessionListComponent = ref(null);


const updateData = () => {
  console.log("update trigger")
  statisticsComponent.value.refresh();
  sessionListComponent.value.refresh();
}
</script>

<template>
  <Header class="pt-5"></Header>
  <div class="container mx-auto">
    <SessionModal @toggleModal="open = false" @submitSuccess="updateData" :open="open"></SessionModal>
    <Statistics ref="statisticsComponent" class="py-5"></Statistics>
    <SessionList ref="sessionListComponent" @updateList="updateData" @toggleModal="open = !open" class="py-5"></SessionList>
  </div>
  <footer>
    <Footer></Footer>
  </footer>
</template>

<style scoped>

</style>