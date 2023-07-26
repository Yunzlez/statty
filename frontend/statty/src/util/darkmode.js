export {
    DARK,
    LIGHT,
    getCurrentTheme,
    updateTheme,
    getThemeName,
    hasStoredTheme
}

const DARK = 'dark';
const LIGHT = 'light';

function getCurrentTheme() {
    let theme = window.localStorage.getItem("preferredTheme");
    if (theme) {
        return theme;
    }

    let darkModeToggle = window.matchMedia('(prefers-color-scheme: dark)');
    let darkMode = darkModeToggle.matches;
    if (!darkMode) {
        return LIGHT;
    }
    return DARK;
}

function updateTheme(isDarkMode, updateInLocalStorage) {
    let toBe = getThemeName(isDarkMode);
    document.documentElement.setAttribute('data-theme', toBe);
    if (updateInLocalStorage) {
        window.localStorage.setItem("preferredTheme", toBe)
    }

}

function getThemeName(isDarkMode) {
    return isDarkMode ? DARK : LIGHT;
}

function hasStoredTheme() {
    let theme = window.localStorage.getItem("preferredTheme");
    if (theme) {
        return true;
    }
    return false;
}
