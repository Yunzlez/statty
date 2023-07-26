<script>
import ThemeToggle from "./components/ThemeToggle.vue";
import {computed} from "vue";
import {DARK, getCurrentTheme, updateTheme} from "./util/darkmode";


export default {
  name: 'App',
  components: {
    ThemeToggle
  },
  data() {
    return {
      isDarkMode: true
    }
  },
  provide() {
    return {
      darkMode: computed(() => this.isDarkMode),
      toggleDarkMode: this.toggleDarkMode
    }
  },
  methods: {
    toggleDarkMode(updateInLocalStorage) {
      this.isDarkMode = !this.isDarkMode;
      updateTheme(this.isDarkMode, updateInLocalStorage)
    }
  },
  created() {
    let darkMode = getCurrentTheme() === DARK
    updateTheme(darkMode, false)
    this.isDarkMode = darkMode;
  }
}
</script>

<template>
  <router-view></router-view>
</template>

<style scoped>
</style>
