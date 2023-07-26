<template>
  <div class="darkmode-toggle">
    <i class="icon sun"></i>
    <div class="ui toggle checkbox" :class="fomanticDark" >
      <input type="checkbox" :checked="darkMode" @click="toggle()" id="theme-toggle-checkbox">
      <label class="coloring blue"><i class="icon moon" :class="fomanticDark"></i></label>
    </div>
  </div>
</template>

<script>
import {hasStoredTheme} from "../util/darkmode.js";

export default {
  name: "ThemeToggle",
  inject: ['darkMode', 'toggleDarkMode'],
  methods: {
    toggle() {
      this.toggleDarkMode(true);
    }
  },
  computed: {
    fomanticDark() {
      return this.darkMode ? 'inverted' : ''
    }
  },
  created() {
    let darkModeToggle = window.matchMedia('(prefers-color-scheme: dark)');
    darkModeToggle.addEventListener('change', (e) => {
      //only change if no theme is stored - stored prefs get precedence
      if (!hasStoredTheme() && (e.matches !== this.darkMode)) {
        this.toggleDarkMode(false);
      }
    });
    console.log(this.darkMode)
  }
}
</script>

<style scoped>
</style>
