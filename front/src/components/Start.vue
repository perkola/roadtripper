<template lang="jade">
div.intro
    div.intro__content
        h1.intro__title Roadtripper
        div.date-from
            h2 Start date
            dp(name="startdate")
        div.date-to
            h2 End date
            dp(name="enddate")
        button(v-link="{ path: 'plan' }").go-btn Go!
</template>
